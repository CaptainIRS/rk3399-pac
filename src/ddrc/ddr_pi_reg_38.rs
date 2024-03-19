#[doc = "Register `DDR_PI_REG_38` reader"]
pub type R = crate::R<DdrPiReg38Spec>;
#[doc = "Register `DDR_PI_REG_38` writer"]
pub type W = crate::W<DdrPiReg38Spec>;
#[doc = "Field `PI_SEQ7_PAT_MASK` reader - Indicates user-defined command mask for power-on sequence 7."]
pub type PiSeq7PatMaskR = crate::FieldReader<u32>;
#[doc = "Field `PI_SEQ7_PAT_MASK` writer - Indicates user-defined command mask for power-on sequence 7."]
pub type PiSeq7PatMaskW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Indicates user-defined command mask for power-on sequence 7."]
    #[inline(always)]
    pub fn pi_seq7_pat_mask(&self) -> PiSeq7PatMaskR {
        PiSeq7PatMaskR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Indicates user-defined command mask for power-on sequence 7."]
    #[inline(always)]
    #[must_use]
    pub fn pi_seq7_pat_mask(&mut self) -> PiSeq7PatMaskW<DdrPiReg38Spec> {
        PiSeq7PatMaskW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_38::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_38::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg38Spec;
impl crate::RegisterSpec for DdrPiReg38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_38::R`](R) reader structure"]
impl crate::Readable for DdrPiReg38Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_38::W`](W) writer structure"]
impl crate::Writable for DdrPiReg38Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_38 to value 0"]
impl crate::Resettable for DdrPiReg38Spec {
    const RESET_VALUE: u32 = 0;
}
