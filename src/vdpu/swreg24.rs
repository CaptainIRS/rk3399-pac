#[doc = "Register `SWREG24` reader"]
pub type R = crate::R<Swreg24Spec>;
#[doc = "Register `SWREG24` writer"]
pub type W = crate::W<Swreg24Spec>;
#[doc = "Field `SW_ABLED_ST_ADR_2ST` reader - 2st alpha blending start address\n\n1.valid when mask2 is used in alpha blending mode\n\n2.Format of data the same as in PP input.\n\n3.Amount of data is related to mask 2 size or ablend1_scanline\n\ninformed with mask 1 size or with ablend1_scanline if ablend when\n\ncrop flag valid"]
pub type SwAbledStAdr2stR = crate::FieldReader<u32>;
#[doc = "Field `SW_ABLED_ST_ADR_2ST` writer - 2st alpha blending start address\n\n1.valid when mask2 is used in alpha blending mode\n\n2.Format of data the same as in PP input.\n\n3.Amount of data is related to mask 2 size or ablend1_scanline\n\ninformed with mask 1 size or with ablend1_scanline if ablend when\n\ncrop flag valid"]
pub type SwAbledStAdr2stW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 2st alpha blending start address\n\n1.valid when mask2 is used in alpha blending mode\n\n2.Format of data the same as in PP input.\n\n3.Amount of data is related to mask 2 size or ablend1_scanline\n\ninformed with mask 1 size or with ablend1_scanline if ablend when\n\ncrop flag valid"]
    #[inline(always)]
    pub fn sw_abled_st_adr_2st(&self) -> SwAbledStAdr2stR {
        SwAbledStAdr2stR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 2st alpha blending start address\n\n1.valid when mask2 is used in alpha blending mode\n\n2.Format of data the same as in PP input.\n\n3.Amount of data is related to mask 2 size or ablend1_scanline\n\ninformed with mask 1 size or with ablend1_scanline if ablend when\n\ncrop flag valid"]
    #[inline(always)]
    #[must_use]
    pub fn sw_abled_st_adr_2st(&mut self) -> SwAbledStAdr2stW<Swreg24Spec> {
        SwAbledStAdr2stW::new(self, 0)
    }
}
#[doc = "alpha blending base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg24Spec;
impl crate::RegisterSpec for Swreg24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg24::R`](R) reader structure"]
impl crate::Readable for Swreg24Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg24::W`](W) writer structure"]
impl crate::Writable for Swreg24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG24 to value 0"]
impl crate::Resettable for Swreg24Spec {
    const RESET_VALUE: u32 = 0;
}
