#[doc = "Register `DDR_PI_REG_169` reader"]
pub type R = crate::R<DdrPiReg169Spec>;
#[doc = "Register `DDR_PI_REG_169` writer"]
pub type W = crate::W<DdrPiReg169Spec>;
#[doc = "Field `PI_TMOD_F2` reader - Indicates TMOD value in cycles. The suffix '_f2' of the parameter\n\nname is omitted when in non-DFS mode."]
pub type PiTmodF2R = crate::FieldReader;
#[doc = "Field `PI_TMOD_F2` writer - Indicates TMOD value in cycles. The suffix '_f2' of the parameter\n\nname is omitted when in non-DFS mode."]
pub type PiTmodF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indicates TMOD value in cycles. The suffix '_f2' of the parameter\n\nname is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tmod_f2(&self) -> PiTmodF2R {
        PiTmodF2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates TMOD value in cycles. The suffix '_f2' of the parameter\n\nname is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmod_f2(&mut self) -> PiTmodF2W<DdrPiReg169Spec> {
        PiTmodF2W::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 169\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_169::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_169::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg169Spec;
impl crate::RegisterSpec for DdrPiReg169Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_169::R`](R) reader structure"]
impl crate::Readable for DdrPiReg169Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_169::W`](W) writer structure"]
impl crate::Writable for DdrPiReg169Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_169 to value 0"]
impl crate::Resettable for DdrPiReg169Spec {
    const RESET_VALUE: u32 = 0;
}
