use anyhow::Result;
use futures_util::StreamExt;
use r2r;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::types::{DroneCommand, DroneInterface, RpytCommand};

pub struct Node {
    node: r2r::Node,
    drone: Arc<Mutex<Box<dyn DroneInterface>>>,
}

impl Node {
    pub fn new(drone: Box<dyn DroneInterface>) -> Result<Self> {
        let ctx = r2r::Context::create()?;
        let node = r2r::Node::create(ctx, "crazyflie", "")?;

        Ok(Self {
            node,
            drone: Arc::new(Mutex::new(drone)),
        })
    }

    pub async fn spin(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Updated return type
        // Initialize drone
        self.drone.lock().await.init().await?;

        // Publishers
        let state_pub = self.node.create_publisher::<r2r::std_msgs::msg::String>(
            "cf/state",
            r2r::QosProfile::default(),
        )?;

        // Subscribers
        let mut cmd_sub = self.node.subscribe::<r2r::geometry_msgs::msg::Twist>(
            "cf/cmd_rpyt",
            r2r::QosProfile::default(),
        )?;

        // Services
        let mut arm_srv = self
            .node
            .create_service::<r2r::std_srvs::srv::SetBool::Service>("cf/arm")?;

        // Main loop
        loop {
            tokio::select! {
                // Publish state
                _ = tokio::time::sleep(std::time::Duration::from_millis(10)) => {
                    let state = self.drone.lock().await.get_state().await?;
                    let msg = r2r::std_msgs::msg::String {
                        data: serde_json::to_string(&state)?
                    };
                    state_pub.publish(&msg)?;
                }

                // Handle commands
                Some(msg) = cmd_sub.next() => {
                    let cmd = DroneCommand::Rpyt(RpytCommand {
                        roll: msg.angular.x as f32,
                        pitch: msg.angular.y as f32,
                        yaw: msg.angular.z as f32,
                        thrust: (msg.linear.z * 65535.0) as u16,
                    });
                    self.drone.lock().await.send_command(cmd).await?;
                }

                // Handle arm/disarm service
                Some(req) = arm_srv.next() => {
                    let cmd = if req.message.data {
                        DroneCommand::Arm
                    } else {
                        DroneCommand::Disarm
                    };
                    self.drone.lock().await.send_command(cmd).await?;
                    let resp = r2r::std_srvs::srv::SetBool::Response {
                        success: true,
                        message: "Command executed".to_string(),
                    };
                    req.respond(resp)?;
                }
            }
        }
    }
}
