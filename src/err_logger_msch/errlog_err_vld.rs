#[doc = "Register `ERRLOG_ErrVld` reader"]
pub type R = crate::R<ErrlogErrVldSpec>;
#[doc = "Field `ERRVLD` reader - When set to 1, indicates that an error is logged in the ErrLog\n\nregisters."]
pub type ErrvldR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - When set to 1, indicates that an error is logged in the ErrLog\n\nregisters."]
    #[inline(always)]
    pub fn errvld(&self) -> ErrvldR {
        ErrvldR::new((self.bits & 1) != 0)
    }
}
#[doc = "Error staus register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errlog_err_vld::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrlogErrVldSpec;
impl crate::RegisterSpec for ErrlogErrVldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errlog_err_vld::R`](R) reader structure"]
impl crate::Readable for ErrlogErrVldSpec {}
#[doc = "`reset()` method sets ERRLOG_ErrVld to value 0"]
impl crate::Resettable for ErrlogErrVldSpec {
    const RESET_VALUE: u32 = 0;
}
