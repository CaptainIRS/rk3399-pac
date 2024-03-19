#[doc = "Register `DDR_PI_REG_36` reader"]
pub type R = crate::R<DdrPiReg36Spec>;
#[doc = "Register `DDR_PI_REG_36` writer"]
pub type W = crate::W<DdrPiReg36Spec>;
#[doc = "Field `PI_SEQ6_PAT_MASK` reader - Indicates user-defined command mask for power-on sequence 6."]
pub type PiSeq6PatMaskR = crate::FieldReader<u32>;
#[doc = "Field `PI_SEQ6_PAT_MASK` writer - Indicates user-defined command mask for power-on sequence 6."]
pub type PiSeq6PatMaskW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Indicates user-defined command mask for power-on sequence 6."]
    #[inline(always)]
    pub fn pi_seq6_pat_mask(&self) -> PiSeq6PatMaskR {
        PiSeq6PatMaskR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Indicates user-defined command mask for power-on sequence 6."]
    #[inline(always)]
    #[must_use]
    pub fn pi_seq6_pat_mask(&mut self) -> PiSeq6PatMaskW<DdrPiReg36Spec> {
        PiSeq6PatMaskW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_36::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_36::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg36Spec;
impl crate::RegisterSpec for DdrPiReg36Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_36::R`](R) reader structure"]
impl crate::Readable for DdrPiReg36Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_36::W`](W) writer structure"]
impl crate::Writable for DdrPiReg36Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_36 to value 0"]
impl crate::Resettable for DdrPiReg36Spec {
    const RESET_VALUE: u32 = 0;
}
