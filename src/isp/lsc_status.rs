#[doc = "Register `LSC_STATUS` reader"]
pub type R = crate::R<LscStatusSpec>;
#[doc = "Field `lsc_en_status` reader - 0: lens shading correction is\n\ncurrently off 1: lens shading\n\ncorrection is currently on"]
pub type LscEnStatusR = crate::BitReader;
#[doc = "Field `active_table` reader - 0: currently active tables set is\n\ntable set 0 1: currently active tables\n\nset is table set 1"]
pub type ActiveTableR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 0: lens shading correction is\n\ncurrently off 1: lens shading\n\ncorrection is currently on"]
    #[inline(always)]
    pub fn lsc_en_status(&self) -> LscEnStatusR {
        LscEnStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: currently active tables set is\n\ntable set 0 1: currently active tables\n\nset is table set 1"]
    #[inline(always)]
    pub fn active_table(&self) -> ActiveTableR {
        ActiveTableR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Lens shade status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscStatusSpec;
impl crate::RegisterSpec for LscStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_status::R`](R) reader structure"]
impl crate::Readable for LscStatusSpec {}
#[doc = "`reset()` method sets LSC_STATUS to value 0"]
impl crate::Resettable for LscStatusSpec {
    const RESET_VALUE: u32 = 0;
}
