#[doc = "Register `SWREG102` reader"]
pub type R = crate::R<Swreg102Spec>;
#[doc = "Register `SWREG102` writer"]
pub type W = crate::W<Swreg102Spec>;
#[doc = "Field `H264_INIT_REFLIST_DF12` reader - 12st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf12R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DF12` writer - 12st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf12W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DF13` reader - 13st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf13R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DF13` writer - 13st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf13W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DF14` reader - 14st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf14R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DF14` writer - 14st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf14W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DF15` reader - 15st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf15R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DF15` writer - 15st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf15W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 12st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_df12(&self) -> H264InitReflistDf12R {
        H264InitReflistDf12R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 13st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_df13(&self) -> H264InitReflistDf13R {
        H264InitReflistDf13R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 14st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_df14(&self) -> H264InitReflistDf14R {
        H264InitReflistDf14R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 15st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_df15(&self) -> H264InitReflistDf15R {
        H264InitReflistDf15R::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 12st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_df12(&mut self) -> H264InitReflistDf12W<Swreg102Spec> {
        H264InitReflistDf12W::new(self, 0)
    }
    #[doc = "Bits 5:9 - 13st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_df13(&mut self) -> H264InitReflistDf13W<Swreg102Spec> {
        H264InitReflistDf13W::new(self, 5)
    }
    #[doc = "Bits 10:14 - 14st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_df14(&mut self) -> H264InitReflistDf14W<Swreg102Spec> {
        H264InitReflistDf14W::new(self, 10)
    }
    #[doc = "Bits 15:19 - 15st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_df15(&mut self) -> H264InitReflistDf15W<Swreg102Spec> {
        H264InitReflistDf15W::new(self, 15)
    }
}
#[doc = "initial reference picture list related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg102::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg102::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg102Spec;
impl crate::RegisterSpec for Swreg102Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg102::R`](R) reader structure"]
impl crate::Readable for Swreg102Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg102::W`](W) writer structure"]
impl crate::Writable for Swreg102Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG102 to value 0"]
impl crate::Resettable for Swreg102Spec {
    const RESET_VALUE: u32 = 0;
}
