#[doc = "Register `DDR_PI_REG_30` reader"]
pub type R = crate::R<DdrPiReg30Spec>;
#[doc = "Register `DDR_PI_REG_30` writer"]
pub type W = crate::W<DdrPiReg30Spec>;
#[doc = "Field `PI_SEQ3_PAT_MASK` reader - Indicates user-defined command mask for power-on sequence 3."]
pub type PiSeq3PatMaskR = crate::FieldReader<u32>;
#[doc = "Field `PI_SEQ3_PAT_MASK` writer - Indicates user-defined command mask for power-on sequence 3."]
pub type PiSeq3PatMaskW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Indicates user-defined command mask for power-on sequence 3."]
    #[inline(always)]
    pub fn pi_seq3_pat_mask(&self) -> PiSeq3PatMaskR {
        PiSeq3PatMaskR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Indicates user-defined command mask for power-on sequence 3."]
    #[inline(always)]
    #[must_use]
    pub fn pi_seq3_pat_mask(&mut self) -> PiSeq3PatMaskW<DdrPiReg30Spec> {
        PiSeq3PatMaskW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_30::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_30::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg30Spec;
impl crate::RegisterSpec for DdrPiReg30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_30::R`](R) reader structure"]
impl crate::Readable for DdrPiReg30Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_30::W`](W) writer structure"]
impl crate::Writable for DdrPiReg30Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_30 to value 0"]
impl crate::Resettable for DdrPiReg30Spec {
    const RESET_VALUE: u32 = 0;
}
