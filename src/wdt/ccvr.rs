#[doc = "Register `CCVR` reader"]
pub type R = crate::R<CcvrSpec>;
#[doc = "Field `CUR_CNT` reader - Current counter value\n\nThis register, when read, is the current value of the internal\n\ncounter. This value is read coherently whenever it is read"]
pub type CurCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current counter value\n\nThis register, when read, is the current value of the internal\n\ncounter. This value is read coherently whenever it is read"]
    #[inline(always)]
    pub fn cur_cnt(&self) -> CurCntR {
        CurCntR::new(self.bits)
    }
}
#[doc = "Current counter value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccvr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcvrSpec;
impl crate::RegisterSpec for CcvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccvr::R`](R) reader structure"]
impl crate::Readable for CcvrSpec {}
#[doc = "`reset()` method sets CCVR to value 0"]
impl crate::Resettable for CcvrSpec {
    const RESET_VALUE: u32 = 0;
}
