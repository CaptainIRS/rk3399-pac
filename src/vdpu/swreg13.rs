#[doc = "Register `SWREG13` reader"]
pub type R = crate::R<Swreg13Spec>;
#[doc = "Register `SWREG13` writer"]
pub type W = crate::W<Swreg13Spec>;
#[doc = "Field `SW_BOTFLD_C_ST_ADR` reader - input bottom field pp start address for c component"]
pub type SwBotfldCStAdrR = crate::FieldReader<u32>;
#[doc = "Field `SW_BOTFLD_C_ST_ADR` writer - input bottom field pp start address for c component"]
pub type SwBotfldCStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - input bottom field pp start address for c component"]
    #[inline(always)]
    pub fn sw_botfld_c_st_adr(&self) -> SwBotfldCStAdrR {
        SwBotfldCStAdrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - input bottom field pp start address for c component"]
    #[inline(always)]
    #[must_use]
    pub fn sw_botfld_c_st_adr(&mut self) -> SwBotfldCStAdrW<Swreg13Spec> {
        SwBotfldCStAdrW::new(self, 2)
    }
}
#[doc = "PP input picture base for Ch bottom field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg13Spec;
impl crate::RegisterSpec for Swreg13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg13::R`](R) reader structure"]
impl crate::Readable for Swreg13Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg13::W`](W) writer structure"]
impl crate::Writable for Swreg13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG13 to value 0"]
impl crate::Resettable for Swreg13Spec {
    const RESET_VALUE: u32 = 0;
}
