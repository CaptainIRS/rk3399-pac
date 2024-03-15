#[doc = "Register `PI_REG_39` reader"]
pub type R = crate::R<PiReg39Spec>;
#[doc = "Register `PI_REG_39` writer"]
pub type W = crate::W<PiReg39Spec>;
#[doc = "Field `PI_SEQ8_PAT` reader - Indicates user-defined power-on sequence 8."]
pub type PiSeq8PatR = crate::FieldReader<u32>;
#[doc = "Field `PI_SEQ8_PAT` writer - Indicates user-defined power-on sequence 8."]
pub type PiSeq8PatW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Indicates user-defined power-on sequence 8."]
    #[inline(always)]
    pub fn pi_seq8_pat(&self) -> PiSeq8PatR {
        PiSeq8PatR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Indicates user-defined power-on sequence 8."]
    #[inline(always)]
    #[must_use]
    pub fn pi_seq8_pat(&mut self) -> PiSeq8PatW<PiReg39Spec> {
        PiSeq8PatW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 39\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_39::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_39::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg39Spec;
impl crate::RegisterSpec for PiReg39Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_39::R`](R) reader structure"]
impl crate::Readable for PiReg39Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_39::W`](W) writer structure"]
impl crate::Writable for PiReg39Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_39 to value 0"]
impl crate::Resettable for PiReg39Spec {
    const RESET_VALUE: u32 = 0;
}
