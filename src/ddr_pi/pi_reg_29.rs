#[doc = "Register `PI_REG_29` reader"]
pub type R = crate::R<PiReg29Spec>;
#[doc = "Register `PI_REG_29` writer"]
pub type W = crate::W<PiReg29Spec>;
#[doc = "Field `PI_SEQ3_PAT` reader - Indicates user-defined power-on sequence 3."]
pub type PiSeq3PatR = crate::FieldReader<u32>;
#[doc = "Field `PI_SEQ3_PAT` writer - Indicates user-defined power-on sequence 3."]
pub type PiSeq3PatW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Indicates user-defined power-on sequence 3."]
    #[inline(always)]
    pub fn pi_seq3_pat(&self) -> PiSeq3PatR {
        PiSeq3PatR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Indicates user-defined power-on sequence 3."]
    #[inline(always)]
    #[must_use]
    pub fn pi_seq3_pat(&mut self) -> PiSeq3PatW<PiReg29Spec> {
        PiSeq3PatW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_29::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_29::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg29Spec;
impl crate::RegisterSpec for PiReg29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_29::R`](R) reader structure"]
impl crate::Readable for PiReg29Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_29::W`](W) writer structure"]
impl crate::Writable for PiReg29Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_29 to value 0"]
impl crate::Resettable for PiReg29Spec {
    const RESET_VALUE: u32 = 0;
}
