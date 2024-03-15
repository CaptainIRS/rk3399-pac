#[doc = "Register `PI_REG_35` reader"]
pub type R = crate::R<PiReg35Spec>;
#[doc = "Register `PI_REG_35` writer"]
pub type W = crate::W<PiReg35Spec>;
#[doc = "Field `PI_SEQ6_PAT` reader - Indicates user-defined power-on sequence 6."]
pub type PiSeq6PatR = crate::FieldReader<u32>;
#[doc = "Field `PI_SEQ6_PAT` writer - Indicates user-defined power-on sequence 6."]
pub type PiSeq6PatW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Indicates user-defined power-on sequence 6."]
    #[inline(always)]
    pub fn pi_seq6_pat(&self) -> PiSeq6PatR {
        PiSeq6PatR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Indicates user-defined power-on sequence 6."]
    #[inline(always)]
    #[must_use]
    pub fn pi_seq6_pat(&mut self) -> PiSeq6PatW<PiReg35Spec> {
        PiSeq6PatW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_35::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_35::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg35Spec;
impl crate::RegisterSpec for PiReg35Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_35::R`](R) reader structure"]
impl crate::Readable for PiReg35Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_35::W`](W) writer structure"]
impl crate::Writable for PiReg35Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_35 to value 0"]
impl crate::Resettable for PiReg35Spec {
    const RESET_VALUE: u32 = 0;
}
