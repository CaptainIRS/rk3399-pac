#[doc = "Register `CQRMEM` reader"]
pub type R = crate::R<CqrmemSpec>;
#[doc = "Response Mode Error Mask\n\nThis bit is used as in interrupt mask on the device status filed\n\nwhich is received in R1/R1b responses.\n\nBit Value Description (for any bit i):\n\nValue on reset: 4260995200"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Rmem {
    #[doc = "1: When a R1/R1b response is received, with bit i in the device status set, a RED interrupt is generated"]
    B1 = 1,
    #[doc = "0: When a R1/R1b response is received, bit i in the device status is ignored The reset value of this register is set to trigger an interrupt on all 'Error' type bits in the device status. NOTE: Responses to CMD13 (SQS) encode the QSR, so they are ignored by this logic."]
    B0 = 0,
}
impl From<Rmem> for u32 {
    #[inline(always)]
    fn from(variant: Rmem) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rmem {
    type Ux = u32;
}
#[doc = "Field `RMEM` reader - Response Mode Error Mask\n\nThis bit is used as in interrupt mask on the device status filed\n\nwhich is received in R1/R1b responses.\n\nBit Value Description (for any bit i):"]
pub type RmemR = crate::FieldReader<Rmem>;
impl RmemR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rmem> {
        match self.bits {
            1 => Some(Rmem::B1),
            0 => Some(Rmem::B0),
            _ => None,
        }
    }
    #[doc = "When a R1/R1b response is received, with bit i in the device status set, a RED interrupt is generated"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rmem::B1
    }
    #[doc = "When a R1/R1b response is received, bit i in the device status is ignored The reset value of this register is set to trigger an interrupt on all 'Error' type bits in the device status. NOTE: Responses to CMD13 (SQS) encode the QSR, so they are ignored by this logic."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rmem::B0
    }
}
impl R {
    #[doc = "Bits 0:31 - Response Mode Error Mask\n\nThis bit is used as in interrupt mask on the device status filed\n\nwhich is received in R1/R1b responses.\n\nBit Value Description (for any bit i):"]
    #[inline(always)]
    pub fn rmem(&self) -> RmemR {
        RmemR::new(self.bits)
    }
}
#[doc = "Command queueing response mode error mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqrmem::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqrmemSpec;
impl crate::RegisterSpec for CqrmemSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqrmem::R`](R) reader structure"]
impl crate::Readable for CqrmemSpec {}
#[doc = "`reset()` method sets CQRMEM to value 0xfdf9_a080"]
impl crate::Resettable for CqrmemSpec {
    const RESET_VALUE: u32 = 0xfdf9_a080;
}
