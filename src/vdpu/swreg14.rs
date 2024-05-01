#[doc = "Register `SWREG14` reader"]
pub type R = crate::R<Swreg14Spec>;
#[doc = "Register `SWREG14` writer"]
pub type W = crate::W<Swreg14Spec>;
#[doc = "Field `SW_MBCROP_CRDTX` reader - coordinate x used in macroblock crop"]
pub type SwMbcropCrdtxR = crate::FieldReader<u16>;
#[doc = "Field `SW_MBCROP_CRDTX` writer - coordinate x used in macroblock crop"]
pub type SwMbcropCrdtxW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SW_MBCROP_CRDTX_EXT` reader - in order to support jpeg to extend bits"]
pub type SwMbcropCrdtxExtR = crate::FieldReader;
#[doc = "Field `SW_MBCROP_CRDTX_EXT` writer - in order to support jpeg to extend bits"]
pub type SwMbcropCrdtxExtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_MBCROP_CRDTY` reader - coordinate y used in macroblock crop"]
pub type SwMbcropCrdtyR = crate::FieldReader;
#[doc = "Field `SW_MBCROP_CRDTY` writer - coordinate y used in macroblock crop"]
pub type SwMbcropCrdtyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_MBCROP_CRDTY_EXT` reader - in order to support jpeg to extend coordinate y bits"]
pub type SwMbcropCrdtyExtR = crate::FieldReader;
#[doc = "Field `SW_MBCROP_CRDTY_EXT` writer - in order to support jpeg to extend coordinate y bits"]
pub type SwMbcropCrdtyExtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_MDMB_8PIX_FLAG` reader - the most down unrotated MB of input picture just 8 rows pix data"]
pub type SwMdmb8pixFlagR = crate::BitReader;
#[doc = "Field `SW_MDMB_8PIX_FLAG` writer - the most down unrotated MB of input picture just 8 rows pix data"]
pub type SwMdmb8pixFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_MRMB_8PIX_FLAG` reader - the most right unrotated MB of input picture just 8 lines pix data\n\n829PP input picture width is not 16 pixels multiple. Only 8 pixels\n\nof the most right MB of the unrotated input picture is used for\n\nPP input."]
pub type SwMrmb8pixFlagR = crate::BitReader;
#[doc = "Field `SW_MRMB_8PIX_FLAG` writer - the most right unrotated MB of input picture just 8 lines pix data\n\n829PP input picture width is not 16 pixels multiple. Only 8 pixels\n\nof the most right MB of the unrotated input picture is used for\n\nPP input."]
pub type SwMrmb8pixFlagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - coordinate x used in macroblock crop"]
    #[inline(always)]
    pub fn sw_mbcrop_crdtx(&self) -> SwMbcropCrdtxR {
        SwMbcropCrdtxR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:11 - in order to support jpeg to extend bits"]
    #[inline(always)]
    pub fn sw_mbcrop_crdtx_ext(&self) -> SwMbcropCrdtxExtR {
        SwMbcropCrdtxExtR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 16:23 - coordinate y used in macroblock crop"]
    #[inline(always)]
    pub fn sw_mbcrop_crdty(&self) -> SwMbcropCrdtyR {
        SwMbcropCrdtyR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - in order to support jpeg to extend coordinate y bits"]
    #[inline(always)]
    pub fn sw_mbcrop_crdty_ext(&self) -> SwMbcropCrdtyExtR {
        SwMbcropCrdtyExtR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - the most down unrotated MB of input picture just 8 rows pix data"]
    #[inline(always)]
    pub fn sw_mdmb_8pix_flag(&self) -> SwMdmb8pixFlagR {
        SwMdmb8pixFlagR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - the most right unrotated MB of input picture just 8 lines pix data\n\n829PP input picture width is not 16 pixels multiple. Only 8 pixels\n\nof the most right MB of the unrotated input picture is used for\n\nPP input."]
    #[inline(always)]
    pub fn sw_mrmb_8pix_flag(&self) -> SwMrmb8pixFlagR {
        SwMrmb8pixFlagR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - coordinate x used in macroblock crop"]
    #[inline(always)]
    #[must_use]
    pub fn sw_mbcrop_crdtx(&mut self) -> SwMbcropCrdtxW<Swreg14Spec> {
        SwMbcropCrdtxW::new(self, 0)
    }
    #[doc = "Bits 9:11 - in order to support jpeg to extend bits"]
    #[inline(always)]
    #[must_use]
    pub fn sw_mbcrop_crdtx_ext(&mut self) -> SwMbcropCrdtxExtW<Swreg14Spec> {
        SwMbcropCrdtxExtW::new(self, 9)
    }
    #[doc = "Bits 16:23 - coordinate y used in macroblock crop"]
    #[inline(always)]
    #[must_use]
    pub fn sw_mbcrop_crdty(&mut self) -> SwMbcropCrdtyW<Swreg14Spec> {
        SwMbcropCrdtyW::new(self, 16)
    }
    #[doc = "Bits 24:26 - in order to support jpeg to extend coordinate y bits"]
    #[inline(always)]
    #[must_use]
    pub fn sw_mbcrop_crdty_ext(&mut self) -> SwMbcropCrdtyExtW<Swreg14Spec> {
        SwMbcropCrdtyExtW::new(self, 24)
    }
    #[doc = "Bit 28 - the most down unrotated MB of input picture just 8 rows pix data"]
    #[inline(always)]
    #[must_use]
    pub fn sw_mdmb_8pix_flag(&mut self) -> SwMdmb8pixFlagW<Swreg14Spec> {
        SwMdmb8pixFlagW::new(self, 28)
    }
    #[doc = "Bit 29 - the most right unrotated MB of input picture just 8 lines pix data\n\n829PP input picture width is not 16 pixels multiple. Only 8 pixels\n\nof the most right MB of the unrotated input picture is used for\n\nPP input."]
    #[inline(always)]
    #[must_use]
    pub fn sw_mrmb_8pix_flag(&mut self) -> SwMrmb8pixFlagW<Swreg14Spec> {
        SwMrmb8pixFlagW::new(self, 29)
    }
}
#[doc = "coordinate used in macroblock crop\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg14Spec;
impl crate::RegisterSpec for Swreg14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg14::R`](R) reader structure"]
impl crate::Readable for Swreg14Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg14::W`](W) writer structure"]
impl crate::Writable for Swreg14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG14 to value 0"]
impl crate::Resettable for Swreg14Spec {
    const RESET_VALUE: u32 = 0;
}
