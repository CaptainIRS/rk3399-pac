#[doc = "Register `SWREG23` reader"]
pub type R = crate::R<Swreg23Spec>;
#[doc = "Register `SWREG23` writer"]
pub type W = crate::W<Swreg23Spec>;
#[doc = "Field `SW_ABLED_ST_ADR_1ST` reader - 1st alpha blending start address\n\n1.valid when mask1 is used in alpha blending mode\n\n2.Format of data the same as in PP input.\n\n3.Amount of data is related to mask 1 size or ablend1_scanline\n\ninformed with mask 1 size or with ablend1_scanline if ablend when\n\ncrop flag valid"]
pub type SwAbledStAdr1stR = crate::FieldReader<u32>;
#[doc = "Field `SW_ABLED_ST_ADR_1ST` writer - 1st alpha blending start address\n\n1.valid when mask1 is used in alpha blending mode\n\n2.Format of data the same as in PP input.\n\n3.Amount of data is related to mask 1 size or ablend1_scanline\n\ninformed with mask 1 size or with ablend1_scanline if ablend when\n\ncrop flag valid"]
pub type SwAbledStAdr1stW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1st alpha blending start address\n\n1.valid when mask1 is used in alpha blending mode\n\n2.Format of data the same as in PP input.\n\n3.Amount of data is related to mask 1 size or ablend1_scanline\n\ninformed with mask 1 size or with ablend1_scanline if ablend when\n\ncrop flag valid"]
    #[inline(always)]
    pub fn sw_abled_st_adr_1st(&self) -> SwAbledStAdr1stR {
        SwAbledStAdr1stR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1st alpha blending start address\n\n1.valid when mask1 is used in alpha blending mode\n\n2.Format of data the same as in PP input.\n\n3.Amount of data is related to mask 1 size or ablend1_scanline\n\ninformed with mask 1 size or with ablend1_scanline if ablend when\n\ncrop flag valid"]
    #[inline(always)]
    #[must_use]
    pub fn sw_abled_st_adr_1st(&mut self) -> SwAbledStAdr1stW<Swreg23Spec> {
        SwAbledStAdr1stW::new(self, 0)
    }
}
#[doc = "Display width and PP input size extension register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg23Spec;
impl crate::RegisterSpec for Swreg23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg23::R`](R) reader structure"]
impl crate::Readable for Swreg23Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg23::W`](W) writer structure"]
impl crate::Writable for Swreg23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG23 to value 0"]
impl crate::Resettable for Swreg23Spec {
    const RESET_VALUE: u32 = 0;
}
