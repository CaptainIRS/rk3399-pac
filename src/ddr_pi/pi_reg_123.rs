#[doc = "Register `PI_REG_123` reader"]
pub type R = crate::R<PiReg123Spec>;
#[doc = "Register `PI_REG_123` writer"]
pub type W = crate::W<PiReg123Spec>;
#[doc = "Field `PI_TDFI_WDQLVL_MAX` reader - Indicates DFI timing param - max enable to resp (PHY eval)."]
pub type PiTdfiWdqlvlMaxR = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_WDQLVL_MAX` writer - Indicates DFI timing param - max enable to resp (PHY eval)."]
pub type PiTdfiWdqlvlMaxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Indicates DFI timing param - max enable to resp (PHY eval)."]
    #[inline(always)]
    pub fn pi_tdfi_wdqlvl_max(&self) -> PiTdfiWdqlvlMaxR {
        PiTdfiWdqlvlMaxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indicates DFI timing param - max enable to resp (PHY eval)."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wdqlvl_max(&mut self) -> PiTdfiWdqlvlMaxW<PiReg123Spec> {
        PiTdfiWdqlvlMaxW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 123\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_123::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_123::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg123Spec;
impl crate::RegisterSpec for PiReg123Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_123::R`](R) reader structure"]
impl crate::Readable for PiReg123Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_123::W`](W) writer structure"]
impl crate::Writable for PiReg123Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_123 to value 0"]
impl crate::Resettable for PiReg123Spec {
    const RESET_VALUE: u32 = 0;
}
