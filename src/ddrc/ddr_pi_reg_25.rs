#[doc = "Register `DDR_PI_REG_25` reader"]
pub type R = crate::R<DdrPiReg25Spec>;
#[doc = "Register `DDR_PI_REG_25` writer"]
pub type W = crate::W<DdrPiReg25Spec>;
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
    pub fn pi_seq1_pat(&mut self) -> PiSeq1PatW<DdrPiReg25Spec> {
        PiSeq1PatW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg25Spec;
impl crate::RegisterSpec for DdrPiReg25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_25::R`](R) reader structure"]
impl crate::Readable for DdrPiReg25Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_25::W`](W) writer structure"]
impl crate::Writable for DdrPiReg25Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_25 to value 0"]
impl crate::Resettable for DdrPiReg25Spec {
    const RESET_VALUE: u32 = 0;
}
