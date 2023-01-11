/* -------------------------------------------------------------------------- *\
 *             Apache 2.0 License Copyright © 2022 The Aurae Authors          *
 *                                                                            *
 *                +--------------------------------------------+              *
 *                |   █████╗ ██╗   ██╗██████╗  █████╗ ███████╗ |              *
 *                |  ██╔══██╗██║   ██║██╔══██╗██╔══██╗██╔════╝ |              *
 *                |  ███████║██║   ██║██████╔╝███████║█████╗   |              *
 *                |  ██╔══██║██║   ██║██╔══██╗██╔══██║██╔══╝   |              *
 *                |  ██║  ██║╚██████╔╝██║  ██║██║  ██║███████╗ |              *
 *                |  ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝ |              *
 *                +--------------------------------------------+              *
 *                                                                            *
 *                         Distributed Systems Runtime                        *
 *                                                                            *
 * -------------------------------------------------------------------------- *
 *                                                                            *
 *   Licensed under the Apache License, Version 2.0 (the "License");          *
 *   you may not use this file except in compliance with the License.         *
 *   You may obtain a copy of the License at                                  *
 *                                                                            *
 *       http://www.apache.org/licenses/LICENSE-2.0                           *
 *                                                                            *
 *   Unless required by applicable law or agreed to in writing, software      *
 *   distributed under the License is distributed on an "AS IS" BASIS,        *
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. *
 *   See the License for the specific language governing permissions and      *
 *   limitations under the License.                                           *
 *                                                                            *
\* -------------------------------------------------------------------------- */

use std::path::PathBuf;
//use std::net::SocketAddr;

use super::{SystemRuntime, SocketStream};
use crate::{init::{logging, InitError, BANNER, system_runtimes::create_unix_socket_stream}, AURAE_SOCK};
//use crate::init::system_runtimes::create_tcp_socket_stream;
use tonic::async_trait;
use tracing::info;

pub(crate) struct DaemonSystemRuntime;

#[async_trait]
impl SystemRuntime for DaemonSystemRuntime {
    async fn init(self, verbose: bool) -> Result<SocketStream, InitError> {
        println!("{}", BANNER);
        logging::init(verbose, false)?;
        info!("Running as a daemon.");
        // TODO: pass this default through
        create_unix_socket_stream(PathBuf::from(AURAE_SOCK)).await
        //create_tcp_socket_stream("0.0.0.0:8080".parse::<SocketAddr>()?).await
    }
}
