#[doc = "Register `PCIE_CLIENT_MSG_STATUS` reader"]
pub type R = crate::R<PcieClientMsgStatusSpec>;
#[doc = "Message fifo almost full flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlmostFull {
    #[doc = "0: almost full"]
    B0 = 0,
    #[doc = "1: almost full"]
    B1 = 1,
}
impl From<AlmostFull> for bool {
    #[inline(always)]
    fn from(variant: AlmostFull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALMOST_FULL` reader - Message fifo almost full flag"]
pub type AlmostFullR = crate::BitReader<AlmostFull>;
impl AlmostFullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AlmostFull {
        match self.bits {
            false => AlmostFull::B0,
            true => AlmostFull::B1,
        }
    }
    #[doc = "almost full"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AlmostFull::B0
    }
    #[doc = "almost full"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AlmostFull::B1
    }
}
#[doc = "Message fifo empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FifoEmpty {
    #[doc = "0: fifo_empty"]
    B0 = 0,
    #[doc = "1: fifo_empty"]
    B1 = 1,
}
impl From<FifoEmpty> for bool {
    #[inline(always)]
    fn from(variant: FifoEmpty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_EMPTY` reader - Message fifo empty"]
pub type FifoEmptyR = crate::BitReader<FifoEmpty>;
impl FifoEmptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FifoEmpty {
        match self.bits {
            false => FifoEmpty::B0,
            true => FifoEmpty::B1,
        }
    }
    #[doc = "fifo_empty"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FifoEmpty::B0
    }
    #[doc = "fifo_empty"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FifoEmpty::B1
    }
}
#[doc = "Message fifo full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FifoFull {
    #[doc = "0: fifo full"]
    B0 = 0,
    #[doc = "1: fifo full"]
    B1 = 1,
}
impl From<FifoFull> for bool {
    #[inline(always)]
    fn from(variant: FifoFull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_FULL` reader - Message fifo full"]
pub type FifoFullR = crate::BitReader<FifoFull>;
impl FifoFullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FifoFull {
        match self.bits {
            false => FifoFull::B0,
            true => FifoFull::B1,
        }
    }
    #[doc = "fifo full"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FifoFull::B0
    }
    #[doc = "fifo full"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FifoFull::B1
    }
}
#[doc = "Field `SPACE2EMPTY` reader - Space to empty Data length before empty"]
pub type Space2emptyR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Message fifo almost full flag"]
    #[inline(always)]
    pub fn almost_full(&self) -> AlmostFullR {
        AlmostFullR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Message fifo empty"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FifoEmptyR {
        FifoEmptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Message fifo full"]
    #[inline(always)]
    pub fn fifo_full(&self) -> FifoFullR {
        FifoFullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Space to empty Data length before empty"]
    #[inline(always)]
    pub fn space2empty(&self) -> Space2emptyR {
        Space2emptyR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
#[doc = "Message control status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_msg_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientMsgStatusSpec;
impl crate::RegisterSpec for PcieClientMsgStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_msg_status::R`](R) reader structure"]
impl crate::Readable for PcieClientMsgStatusSpec {}
#[doc = "`reset()` method sets PCIE_CLIENT_MSG_STATUS to value 0"]
impl crate::Resettable for PcieClientMsgStatusSpec {
    const RESET_VALUE: u32 = 0;
}
