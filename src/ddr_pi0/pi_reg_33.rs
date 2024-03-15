#[doc = "Register `PI_REG_33` reader"]
pub type R = crate::R<PiReg33Spec>;
#[doc = "Register `PI_REG_33` writer"]
pub type W = crate::W<PiReg33Spec>;
#[doc = "Field `PI_SEQ5_PAT` reader - Indicates user-defined power-on sequence 5."]
pub type PiSeq5PatR = crate::FieldReader<u32>;
#[doc = "Field `PI_SEQ5_PAT` writer - Indicates user-defined power-on sequence 5."]
pub type PiSeq5PatW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Indicates user-defined power-on sequence 5."]
    #[inline(always)]
    pub fn pi_seq5_pat(&self) -> PiSeq5PatR {
        PiSeq5PatR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Indicates user-defined power-on sequence 5."]
    #[inline(always)]
    #[must_use]
    pub fn pi_seq5_pat(&mut self) -> PiSeq5PatW<PiReg33Spec> {
        PiSeq5PatW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_33::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_33::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg33Spec;
impl crate::RegisterSpec for PiReg33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_33::R`](R) reader structure"]
impl crate::Readable for PiReg33Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_33::W`](W) writer structure"]
impl crate::Writable for PiReg33Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_33 to value 0"]
impl crate::Resettable for PiReg33Spec {
    const RESET_VALUE: u32 = 0;
}
