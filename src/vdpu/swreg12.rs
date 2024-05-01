#[doc = "Register `SWREG12` reader"]
pub type R = crate::R<Swreg12Spec>;
#[doc = "Register `SWREG12` writer"]
pub type W = crate::W<Swreg12Spec>;
#[doc = "Field `SW_BOTFLD_Y_ST_ADR` reader - input bottom field pp start address for y component\n\ninput bottom field pp start address for y component"]
pub type SwBotfldYStAdrR = crate::FieldReader<u32>;
#[doc = "Field `SW_BOTFLD_Y_ST_ADR` writer - input bottom field pp start address for y component\n\ninput bottom field pp start address for y component"]
pub type SwBotfldYStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - input bottom field pp start address for y component\n\ninput bottom field pp start address for y component"]
    #[inline(always)]
    pub fn sw_botfld_y_st_adr(&self) -> SwBotfldYStAdrR {
        SwBotfldYStAdrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - input bottom field pp start address for y component\n\ninput bottom field pp start address for y component"]
    #[inline(always)]
    #[must_use]
    pub fn sw_botfld_y_st_adr(&mut self) -> SwBotfldYStAdrW<Swreg12Spec> {
        SwBotfldYStAdrW::new(self, 2)
    }
}
#[doc = "PP input picture base address for Y bottom field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg12Spec;
impl crate::RegisterSpec for Swreg12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg12::R`](R) reader structure"]
impl crate::Readable for Swreg12Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg12::W`](W) writer structure"]
impl crate::Writable for Swreg12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG12 to value 0"]
impl crate::Resettable for Swreg12Spec {
    const RESET_VALUE: u32 = 0;
}
