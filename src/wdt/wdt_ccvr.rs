#[doc = "Register `WDT_CCVR` reader"]
pub type R = crate::R<WdtCcvrSpec>;
#[doc = "Field `CUR_CNT` reader - Current counter value\n\nThis register, when read, is the current value of the internal\n\ncounter. This value is read coherently whenever it is read"]
pub type CurCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current counter value\n\nThis register, when read, is the current value of the internal\n\ncounter. This value is read coherently whenever it is read"]
    #[inline(always)]
    pub fn cur_cnt(&self) -> CurCntR {
        CurCntR::new(self.bits)
    }
}
#[doc = "Current counter value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_ccvr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtCcvrSpec;
impl crate::RegisterSpec for WdtCcvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt_ccvr::R`](R) reader structure"]
impl crate::Readable for WdtCcvrSpec {}
#[doc = "`reset()` method sets WDT_CCVR to value 0"]
impl crate::Resettable for WdtCcvrSpec {
    const RESET_VALUE: u32 = 0;
}
