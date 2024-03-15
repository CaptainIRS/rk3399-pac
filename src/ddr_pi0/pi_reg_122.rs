#[doc = "Register `PI_REG_122` reader"]
pub type R = crate::R<PiReg122Spec>;
#[doc = "Register `PI_REG_122` writer"]
pub type W = crate::W<PiReg122Spec>;
#[doc = "Field `PI_TDFI_WDQLVL_RESP` reader - Indicates DFI timing param - req to enable."]
pub type PiTdfiWdqlvlRespR = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_WDQLVL_RESP` writer - Indicates DFI timing param - req to enable."]
pub type PiTdfiWdqlvlRespW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Indicates DFI timing param - req to enable."]
    #[inline(always)]
    pub fn pi_tdfi_wdqlvl_resp(&self) -> PiTdfiWdqlvlRespR {
        PiTdfiWdqlvlRespR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indicates DFI timing param - req to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wdqlvl_resp(&mut self) -> PiTdfiWdqlvlRespW<PiReg122Spec> {
        PiTdfiWdqlvlRespW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 122\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_122::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_122::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg122Spec;
impl crate::RegisterSpec for PiReg122Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_122::R`](R) reader structure"]
impl crate::Readable for PiReg122Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_122::W`](W) writer structure"]
impl crate::Writable for PiReg122Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_122 to value 0"]
impl crate::Resettable for PiReg122Spec {
    const RESET_VALUE: u32 = 0;
}
