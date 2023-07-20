use std::convert::From;
use super::subscribe::Sub;
use std::future::Future;
use std::pin::Pin;


pub enum CmdType<MSG> {
    Normal,
    Browser,
    WebSocket,
    Http,
    JavaScript,
    Grpc(Pin<Box<dyn Future<Output = MSG>>>),
    GraphQL
}


pub struct Cmd<MSG> {
    pub messages: Vec<(CmdType<MSG>, MSG)>
}

impl<MSG> Cmd<MSG> {
    fn new(
        messages: impl IntoIterator<Item = (CmdType<MSG>, MSG)>
    ) -> Self {

        Self {
            messages: messages.into_iter().collect()
        }
    }

    pub fn normal(msg: MSG) -> Self {

        Self {
            messages: vec![(CmdType::Normal, msg)]
        }
    }

    pub fn normal_batch(
        messages: impl IntoIterator<Item = MSG>
    ) -> Self {

        Self {
            messages: messages.into_iter().map(|msg| (CmdType::Normal, msg)).collect()
        }
    }

    pub fn browser(msg: MSG) -> Self {

        Self {
            messages: vec![(CmdType::Browser, msg)]
        }
    }

    pub fn browser_batch(
        messages: impl IntoIterator<Item = MSG>
    ) -> Self {

        Self {
            messages: messages.into_iter().map(|msg| (CmdType::Browser, msg)).collect()
        }
    }

    pub fn websocket(msg: MSG) -> Self {

        Self {
            messages: vec![(CmdType::WebSocket, msg)]
        }
    }

    pub fn websocket_batch(
        messages: impl IntoIterator<Item = MSG>
    ) -> Self {

        Self {
            messages: messages.into_iter().map(|msg| (CmdType::WebSocket, msg)).collect()
        }
    }

    pub fn http(msg: MSG) -> Self {

        Self {
            messages: vec![(CmdType::Http, msg)]
        }
    }

    pub fn http_batch(
        messages: impl IntoIterator<Item = MSG>
    ) -> Self {

        Self {
            messages: messages.into_iter().map(|msg| (CmdType::Http, msg)).collect()
        }
    }

    pub fn javascript(msg: MSG) -> Self {

        Self {
            messages: vec![(CmdType::JavaScript, msg)]
        }
    }

    pub fn javascript_batch(
        messages: impl IntoIterator<Item = MSG>
    ) -> Self {

        Self {
            messages: messages.into_iter().map(|msg| (CmdType::JavaScript, msg)).collect()
        }
    }

    pub fn grpc(f: Pin<Box<dyn Future<Output = MSG>>>, msg: MSG) -> Self 
    {
        Self {
            messages: vec![(CmdType::Grpc(f), msg)]
        }
    }

    pub fn graphql(msg: MSG) -> Self {

        Self {
            messages: vec![(CmdType::GraphQL, msg)]
        }
    }

    pub fn graphql_batch(
        messages: impl IntoIterator<Item = MSG>
    ) -> Self {

        Self {
            messages: messages.into_iter().map(|msg| (CmdType::GraphQL, msg)).collect()
        }
    }

    pub fn none() -> Self {
        Self {
            messages: Vec::with_capacity(0),
        }
    }

    pub fn map_cmd_msg<MSG2>(self) -> Cmd<MSG2>
    where
        MSG: 'static,
        MSG2: From<MSG>
    {
        let Cmd {
            messages
        } = self;

        Cmd {
            messages: messages.into_iter().map(|(cmd_type, msg)| {
                let local_cmd_type = match cmd_type {
                    CmdType::Grpc(gr) => {
                        CmdType::Grpc(Box::pin(async {
                            MSG2::from(gr.await)
                        }))
                    },
                    CmdType::Normal => CmdType::<MSG2>::Normal,
                    CmdType::Browser => CmdType::<MSG2>::Browser,
                    CmdType::WebSocket => CmdType::<MSG2>::WebSocket,
                    CmdType::Http => CmdType::<MSG2>::Http,
                    CmdType::JavaScript => CmdType::<MSG2>::JavaScript,
                    CmdType::GraphQL => CmdType::<MSG2>::GraphQL,
                };

                (local_cmd_type, MSG2::from(msg))
            }).collect()
        }
    }

    pub fn append_cmd(
        mut self,
        messages: impl IntoIterator<Item = (CmdType<MSG>, MSG)>
    ) -> Self {
        self.messages.extend(messages);
        self
    }

    pub fn merge_all(all_cmds: Vec<Self>) -> Self {
        let mut messages = vec![];
        for cmd in all_cmds {
            messages.extend(cmd.messages);
        }
        Cmd::new(messages)
    }
    
    pub fn extend(
        mut self,
        messages: impl IntoIterator<Item = (CmdType<MSG>, MSG)>
    ) -> Self {
        self.messages.extend(messages);
        self
    }
}


impl<MSG> From<Sub<MSG>> for Cmd<MSG> {
    fn from(item: Sub<MSG>) -> Self {
        let messages = item.messages.into_iter().map(|msg| (CmdType::Normal, msg)).collect();
        Cmd {
            messages
        }
    }
}