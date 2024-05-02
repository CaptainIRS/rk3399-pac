#[doc = "Register `VSM_DELTA_H` reader"]
pub type R = crate::R<VsmDeltaHSpec>;
#[doc = "Field `delta_h` reader - 12Bit two's\n\ncomplement. positive values indicate a displacement of\n\nthe image from right to left (camera turns right)\n\n\n\n"]
pub type DeltaHR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - 12Bit two's\n\ncomplement. positive values indicate a displacement of\n\nthe image from right to left (camera turns right)\n\n\n\n"]
    #[inline(always)]
    pub fn delta_h(&self) -> DeltaHR {
        DeltaHR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "estimated horizontal displacement\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vsm_delta_h::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VsmDeltaHSpec;
impl crate::RegisterSpec for VsmDeltaHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vsm_delta_h::R`](R) reader structure"]
impl crate::Readable for VsmDeltaHSpec {}
#[doc = "`reset()` method sets VSM_DELTA_H to value 0"]
impl crate::Resettable for VsmDeltaHSpec {
    const RESET_VALUE: u32 = 0;
}
