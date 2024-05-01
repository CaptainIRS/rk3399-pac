#[doc = "Register `SWREG101` reader"]
pub type R = crate::R<Swreg101Spec>;
#[doc = "Register `SWREG101` writer"]
pub type W = crate::W<Swreg101Spec>;
#[doc = "Field `H264_INIT_REFLIST_DF6` reader - 6st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf6R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DF6` writer - 6st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DF7` reader - 7st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf7R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DF7` writer - 7st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DF8` reader - 8st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf8R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DF8` writer - 8st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf8W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DF9` reader - 9st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf9R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DF9` writer - 9st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf9W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DF10` reader - 10st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf10R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DF10` writer - 10st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf10W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DF11` reader - 11st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf11R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DF11` writer - 11st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf11W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 6st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_df6(&self) -> H264InitReflistDf6R {
        H264InitReflistDf6R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 7st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_df7(&self) -> H264InitReflistDf7R {
        H264InitReflistDf7R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 8st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_df8(&self) -> H264InitReflistDf8R {
        H264InitReflistDf8R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 9st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_df9(&self) -> H264InitReflistDf9R {
        H264InitReflistDf9R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 10st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_df10(&self) -> H264InitReflistDf10R {
        H264InitReflistDf10R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - 11st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_df11(&self) -> H264InitReflistDf11R {
        H264InitReflistDf11R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 6st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_df6(&mut self) -> H264InitReflistDf6W<Swreg101Spec> {
        H264InitReflistDf6W::new(self, 0)
    }
    #[doc = "Bits 5:9 - 7st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_df7(&mut self) -> H264InitReflistDf7W<Swreg101Spec> {
        H264InitReflistDf7W::new(self, 5)
    }
    #[doc = "Bits 10:14 - 8st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_df8(&mut self) -> H264InitReflistDf8W<Swreg101Spec> {
        H264InitReflistDf8W::new(self, 10)
    }
    #[doc = "Bits 15:19 - 9st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_df9(&mut self) -> H264InitReflistDf9W<Swreg101Spec> {
        H264InitReflistDf9W::new(self, 15)
    }
    #[doc = "Bits 20:24 - 10st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_df10(&mut self) -> H264InitReflistDf10W<Swreg101Spec> {
        H264InitReflistDf10W::new(self, 20)
    }
    #[doc = "Bits 25:29 - 11st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_df11(&mut self) -> H264InitReflistDf11W<Swreg101Spec> {
        H264InitReflistDf11W::new(self, 25)
    }
}
#[doc = "initial reference picture list related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg101::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg101::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg101Spec;
impl crate::RegisterSpec for Swreg101Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg101::R`](R) reader structure"]
impl crate::Readable for Swreg101Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg101::W`](W) writer structure"]
impl crate::Writable for Swreg101Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG101 to value 0"]
impl crate::Resettable for Swreg101Spec {
    const RESET_VALUE: u32 = 0;
}
