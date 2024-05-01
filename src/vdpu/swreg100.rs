#[doc = "Register `SWREG100` reader"]
pub type R = crate::R<Swreg100Spec>;
#[doc = "Register `SWREG100` writer"]
pub type W = crate::W<Swreg100Spec>;
#[doc = "Field `H264_INIT_REFLIST_DF0` reader - 0st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf0R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DF0` writer - 0st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DF1` reader - 1st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf1R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DF1` writer - 1st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DF2` reader - 2st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf2R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DF2` writer - 2st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DF3` reader - 3st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf3R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DF3` writer - 3st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DF4` reader - 4st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf4R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DF4` writer - 4st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DF5` reader - 5st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf5R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DF5` writer - 5st initial reference picture list for direct forward picid\n\nused for h264"]
pub type H264InitReflistDf5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 0st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_df0(&self) -> H264InitReflistDf0R {
        H264InitReflistDf0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 1st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_df1(&self) -> H264InitReflistDf1R {
        H264InitReflistDf1R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 2st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_df2(&self) -> H264InitReflistDf2R {
        H264InitReflistDf2R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 3st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_df3(&self) -> H264InitReflistDf3R {
        H264InitReflistDf3R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 4st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_df4(&self) -> H264InitReflistDf4R {
        H264InitReflistDf4R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - 5st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_df5(&self) -> H264InitReflistDf5R {
        H264InitReflistDf5R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 0st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_df0(&mut self) -> H264InitReflistDf0W<Swreg100Spec> {
        H264InitReflistDf0W::new(self, 0)
    }
    #[doc = "Bits 5:9 - 1st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_df1(&mut self) -> H264InitReflistDf1W<Swreg100Spec> {
        H264InitReflistDf1W::new(self, 5)
    }
    #[doc = "Bits 10:14 - 2st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_df2(&mut self) -> H264InitReflistDf2W<Swreg100Spec> {
        H264InitReflistDf2W::new(self, 10)
    }
    #[doc = "Bits 15:19 - 3st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_df3(&mut self) -> H264InitReflistDf3W<Swreg100Spec> {
        H264InitReflistDf3W::new(self, 15)
    }
    #[doc = "Bits 20:24 - 4st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_df4(&mut self) -> H264InitReflistDf4W<Swreg100Spec> {
        H264InitReflistDf4W::new(self, 20)
    }
    #[doc = "Bits 25:29 - 5st initial reference picture list for direct forward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_df5(&mut self) -> H264InitReflistDf5W<Swreg100Spec> {
        H264InitReflistDf5W::new(self, 25)
    }
}
#[doc = "initial reference picture list related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg100::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg100::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg100Spec;
impl crate::RegisterSpec for Swreg100Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg100::R`](R) reader structure"]
impl crate::Readable for Swreg100Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg100::W`](W) writer structure"]
impl crate::Writable for Swreg100Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG100 to value 0"]
impl crate::Resettable for Swreg100Spec {
    const RESET_VALUE: u32 = 0;
}
