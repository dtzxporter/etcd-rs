use super::{DeleteRequest, KeyRange, PutRequest, RangeRequest};
use crate::proto::etcdserverpb;
use etcdserverpb::compare::{CompareResult, CompareTarget, TargetUnion};
use etcdserverpb::Compare;

pub struct TxnRequest {
    proto: etcdserverpb::TxnRequest,
}

impl TxnRequest {
    pub fn new() -> Self {
        Self {
            proto: etcdserverpb::TxnRequest {
                compare: vec![],
                success: vec![],
                failure: vec![],
            },
        }
    }

    /// Add a version compare
    pub fn when_version(mut self, key_range: KeyRange, cmp: TxnCmp, version: usize) -> Self {
        let result: CompareResult = cmp.into();
        self.proto.compare.push(Compare {
            result: result as i32,
            target: CompareTarget::Version as i32,
            key: key_range.key,
            range_end: key_range.range_end,
            target_union: Some(TargetUnion::Version(version as i64)),
        });
        self
    }

    /// Add a create revision compare
    pub fn when_create_revision(
        mut self,
        key_range: KeyRange,
        cmp: TxnCmp,
        revision: usize,
    ) -> Self {
        let result: CompareResult = cmp.into();
        self.proto.compare.push(Compare {
            result: result as i32,
            target: CompareTarget::Create as i32,
            key: key_range.key,
            range_end: key_range.range_end,
            target_union: Some(TargetUnion::CreateRevision(revision as i64)),
        });
        self
    }

    /// Add a mod revision compare
    pub fn when_mod_revision(mut self, key_range: KeyRange, cmp: TxnCmp, revision: usize) -> Self {
        let result: CompareResult = cmp.into();
        self.proto.compare.push(Compare {
            result: result as i32,
            target: CompareTarget::Mod as i32,
            key: key_range.key,
            range_end: key_range.range_end,
            target_union: Some(TargetUnion::ModRevision(revision as i64)),
        });
        self
    }

    /// Add a value compare
    pub fn when_value<V>(mut self, key_range: KeyRange, cmp: TxnCmp, value: V) -> Self
    where
        V: Into<Vec<u8>>,
    {
        let result: CompareResult = cmp.into();
        self.proto.compare.push(Compare {
            result: result as i32,
            target: CompareTarget::Value as i32,
            key: key_range.key,
            range_end: key_range.range_end,
            target_union: Some(TargetUnion::Value(value.into())),
        });
        self
    }

    /// If compare success, then execute
    pub fn and_then<O>(mut self, op: O) -> Self
    where
        O: Into<TxnOp>,
    {
        self.proto.success.push(op.into().into());
        self
    }

    /// If compare fail, then execute
    pub fn or_else<O>(mut self, op: O) -> Self
    where
        O: Into<TxnOp>,
    {
        self.proto.failure.push(op.into().into());
        self
    }
}

impl Into<etcdserverpb::TxnRequest> for TxnRequest {
    fn into(self) -> etcdserverpb::TxnRequest {
        self.proto
    }
}

/// Transaction Operation
pub enum TxnOp {
    Range(RangeRequest),
    Put(PutRequest),
    Delete(DeleteRequest),
    Txn(TxnRequest),
}

impl Into<etcdserverpb::RequestOp> for TxnOp {
    fn into(self) -> etcdserverpb::RequestOp {
        use etcdserverpb::request_op::Request;

        let req = match self {
            Self::Range(req) => Request::RequestRange(req.into()),
            Self::Put(req) => Request::RequestPut(req.into()),
            Self::Delete(req) => Request::RequestDeleteRange(req.into()),
            Self::Txn(req) => Request::RequestTxn(req.into()),
        };

        etcdserverpb::RequestOp { request: Some(req) }
    }
}

impl From<RangeRequest> for TxnOp {
    fn from(req: RangeRequest) -> Self {
        Self::Range(req)
    }
}

impl From<PutRequest> for TxnOp {
    fn from(req: PutRequest) -> Self {
        Self::Put(req)
    }
}

impl From<DeleteRequest> for TxnOp {
    fn from(req: DeleteRequest) -> Self {
        Self::Delete(req)
    }
}

impl From<TxnRequest> for TxnOp {
    fn from(req: TxnRequest) -> Self {
        Self::Txn(req)
    }
}

/// Transaction Comapre
pub enum TxnCmp {
    Equal,
    NotEqual,
    Greater,
    Less,
}

impl Into<CompareResult> for TxnCmp {
    fn into(self) -> CompareResult {
        match self {
            TxnCmp::Equal => CompareResult::Equal,
            TxnCmp::NotEqual => CompareResult::NotEqual,
            TxnCmp::Greater => CompareResult::Greater,
            TxnCmp::Less => CompareResult::Less,
        }
    }
}

#[derive(Debug)]
pub struct TxnResponse {
    proto: etcdserverpb::TxnResponse,
}

impl From<etcdserverpb::TxnResponse> for TxnResponse {
    fn from(resp: etcdserverpb::TxnResponse) -> Self {
        Self { proto: resp }
    }
}
