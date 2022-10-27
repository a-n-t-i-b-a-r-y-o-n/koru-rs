use crate::device::Device;
use ecp::{ContentData, Get};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct App {
    pub id: i32,
    pub apptype: String,
    pub version: String,
    pub name: String,
    pub icon: Option<Vec<u8>>,
}

impl App {
    // Download the icon for this app from the device, then update this instance of App
    pub async fn fetch_icon(&mut self, parent_device: &mut Device) {
        if let Some(connection) = &mut parent_device.connection {
            if let Some(response) = connection.send_request(Get::QueryAppIcon { channel_id: self.id }.into()).await {
                if let Some(ContentData::Data { bytes: data }) = response.content_data {
                    self.icon = Some(data);
                }
            }
        }
    }
}