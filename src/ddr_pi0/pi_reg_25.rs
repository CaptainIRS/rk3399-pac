#[doc = "Register `PI_REG_25` reader"]
pub type R = crate::R<PiReg25Spec>;
#[doc = "Register `PI_REG_25` writer"]
pub type W = crate::W<PiReg25Spec>;
#[doc = "Field `PI_SEQ1_PAT` reader - Indicates user-defined power-on sequence 1."]
pub type PiSeq1PatR = crate::FieldReader<u32>;
#[doc = "Field `PI_SEQ1_PAT` writer - Indicates user-defined power-on sequence 1."]
pub type PiSeq1PatW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Indicates user-defined power-on sequence 1."]
    #[inline(always)]
    pub fn pi_seq1_pat(&self) -> PiSeq1PatR {
        PiSeq1PatR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Indicates user-defined power-on sequence 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_seq1_pat(&mut self) -> PiSeq1PatW<PiReg25Spec> {
        PiSeq1PatW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg25Spec;
impl crate::RegisterSpec for PiReg25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_25::R`](R) reader structure"]
impl crate::Readable for PiReg25Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_25::W`](W) writer structure"]
impl crate::Writable for PiReg25Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_25 to value 0"]
impl crate::Resettable for PiReg25Spec {
    const RESET_VALUE: u32 = 0;
}
