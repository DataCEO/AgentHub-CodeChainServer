
use super::super::agent::SendAgentRPC;
use super::super::common_rpc_types::{NodeName, ShellStartCodeChainRequest};
use super::super::router::Router;
use super::super::rpc::{response, RPCError, RPCResponse};
use super::types::{
    Context, DashboardGetNetworkResponse, DashboardNode, LogGetRequest, LogGetResponse, LogGetTargetsResponse,
    NodeConnection, NodeGetInfoResponse,
};
use common_rpc_types::UpdateCodeChainRequest;

pub fn add_routing(router: &mut Router<Context>) {
    router.add_route("ping", Box::new(ping as fn(Context) -> RPCResponse<String>));
    router.add_route(
        "node_getInfo",
        Box::new(node_get_info as fn(Context, (String,)) -> RPCResponse<NodeGetInfoResponse>),
    );
    router.add_route(
        "dashboard_getNetwork",
        Box::new(dashboard_get_network as fn(Context) -> RPCResponse<DashboardGetNetworkResponse>),
    );
    router.add_route(