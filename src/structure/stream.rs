use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Operation {
    auth,
    subscribe,
    ping,
    pong,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Topic {
    position,
    execution,
    order,
    wallet,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Category {
    spot,
    linear,
    inverse,
    option,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PositionStatus {
    Normal,
    Liq,
    Adl,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum StopOrderType {
    TakeProfit,
    StopLoss,
    TrailingStop,
    Stop,
    PartialTakeProfit,
    PartialStopLoss,
    tpslOrder,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ExecutionType {
    Trade,
    AdlTrade,
    Funding,
    BustTrade,
    Delivery,
    BlockTrade,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum OrderType {
    Market,
    Limit,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CancelType {
    CancelByUser,
    CancelByReduceOnly,
    CancelByPrepareLiq,
    CancelAllBeforeLiq,
    CancelByPrepareAdl,
    CancelAllBeforeAdl,
    CancelByAdmin,
    CancelByTpSlTsClear,
    CancelByPzSideCh,
    CancelBySmp,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RejectReason {
    EC_NoError,
    EC_Others,
    EC_UnknownMessageType,
    EC_MissingClOrdID,
    EC_MissingOrigClOrdID,
    EC_ClOrdIDOrigClOrdIDAreTheSame,
    EC_DuplicatedClOrdID,
    EC_OrigClOrdIDDoesNotExist,
    EC_TooLateToCancel,
    EC_UnknownOrderType,
    EC_UnknownSide,
    EC_UnknownTimeInForce,
    EC_WronglyRouted,
    EC_MarketOrderPriceIsNotZero,
    EC_LimitOrderInvalidPrice,
    EC_NoEnoughQtyToFill,
    EC_NoImmediateQtyToFill,
    EC_PerCancelRequest,
    EC_MarketOrderCannotBePostOnly,
    EC_PostOnlyWillTakeLiquidity,
    EC_CancelReplaceOrder,
    EC_InvalidSymbolStatus,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum OrderStatus {
    Created,
    New,
    Rejected,
    PartiallyFilled,
    PartiallyFilledCanceled,
    Filled,
    Cancelled,
    Untriggered,
    Triggered,
    Deactivated,
    Active,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TriggerBy {
    LastPrice,
    IndexPrice,
    MarkPrice,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SelfMatchPreventionType {
    None,
    CancelMaker,
    CancelTaker,
    CancelBoth,
}

#[derive(Serialize)]
pub struct Request {
    pub req_id: String,
    pub op: Operation,
    pub args: Vec<String>,
}

#[derive(Serialize)]
pub struct Ping {
    pub req_id: String,
    pub op: Operation,
}

#[derive(Debug, Deserialize)]
pub struct SuccessResponse {
    pub req_id: String,
    pub success: bool,
    pub ret_msg: String,
    pub op: Operation,
    pub conn_id: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct Position {
    pub positionIdx: u32,
    pub tradeMode: u64,
    pub riskId: u64,
    pub riskLimitValue: String,
    pub symbol: String,
    pub side: String,
    pub size: String,
    pub entryPrice: String,
    pub leverage: String,
    pub positionValue: String,
    pub positionBalance: String,
    pub markPrice: String,
    pub positionIM: String,
    pub positionMM: String,
    pub takeProfit: String,
    pub stopLoss: String,
    pub trailingStop: String,
    pub unrealisedPnl: String,
    pub cumRealisedPnl: String,
    pub createdTime: String,
    pub updatedTime: String,
    pub tpslMode: String,
    pub liqPrice: String,
    pub bustPrice: String,
    pub category: Category,
    pub positionStatus: PositionStatus,
    pub adlRankIndicator: u32,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct PositionChannel {
    pub id: String,
    pub topic: Topic,
    pub creationTime: u64,
    pub data: Vec<Position>,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Execution {
    pub category: Category,
    pub symbol: String,
    pub execFee: String,
    pub execId: String,
    pub execPrice: String,
    pub execQty: String,
    pub execType: ExecutionType,
    pub execValue: String,
    pub isMaker: bool,
    pub feeRate: String,
    pub tradeIv: String,
    pub markIv: String,
    pub blockTradeId: String,
    pub markPrice: String,
    pub indexPrice: String,
    pub underlyingPrice: String,
    pub leavesQty: String,
    pub orderId: String,
    pub orderLinkId: String,
    pub orderPrice: String,
    pub orderQty: String,
    pub orderType: OrderType,
    pub stopOrderType: StopOrderType,
    pub side: String,
    pub execTime: String,
    pub isLeverage: String,
    pub closedSize: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct ExecutionChannel {
    pub id: String,
    pub topic: Topic,
    pub creationTime: u64,
    pub data: Vec<Execution>,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Order {
    pub symbol: String,
    pub orderId: String,
    pub side: String,
    pub orderType: OrderType,
    pub cancelType: CancelType,
    pub price: String,
    pub qty: String,
    pub orderIv: String,
    pub timeInForce: TimeInForce,
    pub orderStatus: OrderStatus,
    pub orderLinkId: String,
    pub lastPriceOnCreated: String,
    pub reduceOnly: bool,
    pub leavesQty: String,
    pub leavesValue: String,
    pub cumExecQty: String,
    pub cumExecValue: String,
    pub avgPrice: String,
    pub blockTradeId: String,
    pub positionIdx: String,
    pub cumExecFee: String,
    pub createdTime: String,
    pub updatedTime: String,
    pub rejectReason: RejectReason,
    pub stopOrderType: StopOrderType,
    pub tpslMode: String,
    pub triggerPrice: String,
    pub takeProfit: String,
    pub stopLoss: String,
    pub tpTriggerBy: TriggerBy,
    pub slTriggerBy: TriggerBy,
    pub tpLimitPrice: String,
    pub slLimitPrice: String,
    pub triggerDirection: u32,
    pub triggerBy: TriggerBy,
    pub closeOnTrigger: bool,
    pub category: Category,
    pub placeType: String,
    pub smpType: SelfMatchPreventionType,
    pub smpGroup: u32,
    pub smpOrderId: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct OrderChannel {
    pub id: String,
    pub topic: Topic,
    pub creationTime: u64,
    pub data: Vec<Order>,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct Coin {
    pub coin: String,
    pub equity: String,
    pub usdValue: String,
    pub walletBalance: String,
    pub availableToWithdraw: String,
    pub availableToBorrow: String,
    pub borrowAmount: String,
    pub accruedInterest: String,
    pub totalOrderIM: String,
    pub totalPositionIM: String,
    pub totalPositionMM: String,
    pub unrealisedPnl: String,
    pub cumRealisedPnl: String,
    pub bonus: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct Wallet {
    pub accountType: String,
    pub accountLTV: String,
    pub accountIMRate: String,
    pub accountMMRate: String,
    pub totalEquity: String,
    pub totalWalletBalance: String,
    pub totalMarginBalance: String,
    pub totalAvailableBalance: String,
    pub totalPerpUPL: String,
    pub totalInitialMargin: String,
    pub totalMaintenanceMargin: String,
    pub coin: Vec<Coin>,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct WalletChannel {
    pub id: String,
    pub topic: Topic,
    pub creationTime: u64,
    pub data: Vec<Wallet>,
}
