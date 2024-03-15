#[doc = "Register `PI_REG_40` reader"]
pub type R = crate::R<PiReg40Spec>;
#[doc = "Register `PI_REG_40` writer"]
pub type W = crate::W<PiReg40Spec>;
#[doc = "Field `PI_SEQ8_PAT_MASK` reader - Indicates user-defined command mask for power-on sequence 8."]
pub type PiSeq8PatMaskR = crate::FieldReader<u32>;
#[doc = "Field `PI_SEQ8_PAT_MASK` writer - Indicates user-defined command mask for power-on sequence 8."]
pub type PiSeq8PatMaskW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Indicates user-defined command mask for power-on sequence 8."]
    #[inline(always)]
    pub fn pi_seq8_pat_mask(&self) -> PiSeq8PatMaskR {
        PiSeq8PatMaskR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Indicates user-defined command mask for power-on sequence 8."]
    #[inline(always)]
    #[must_use]
    pub fn pi_seq8_pat_mask(&mut self) -> PiSeq8PatMaskW<PiReg40Spec> {
        PiSeq8PatMaskW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_40::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_40::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg40Spec;
impl crate::RegisterSpec for PiReg40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_40::R`](R) reader structure"]
impl crate::Readable for PiReg40Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_40::W`](W) writer structure"]
impl crate::Writable for PiReg40Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_40 to value 0"]
impl crate::Resettable for PiReg40Spec {
    const RESET_VALUE: u32 = 0;
}
