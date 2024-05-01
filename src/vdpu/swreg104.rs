#[doc = "Register `SWREG104` reader"]
pub type R = crate::R<Swreg104Spec>;
#[doc = "Register `SWREG104` writer"]
pub type W = crate::W<Swreg104Spec>;
#[doc = "Field `H264_INIT_REFLIST_DB6` reader - 6st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb6R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DB6` writer - 6st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DB7` reader - 7st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb7R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DB7` writer - 7st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DB8` reader - 8st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb8R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DB8` writer - 8st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb8W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DB9` reader - 9st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb9R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DB9` writer - 9st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb9W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DB10` reader - 10st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb10R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DB10` writer - 10st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb10W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DB11` reader - 11st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb11R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DB11` writer - 11st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb11W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 6st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_db6(&self) -> H264InitReflistDb6R {
        H264InitReflistDb6R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 7st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_db7(&self) -> H264InitReflistDb7R {
        H264InitReflistDb7R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 8st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_db8(&self) -> H264InitReflistDb8R {
        H264InitReflistDb8R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 9st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_db9(&self) -> H264InitReflistDb9R {
        H264InitReflistDb9R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 10st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_db10(&self) -> H264InitReflistDb10R {
        H264InitReflistDb10R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - 11st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_db11(&self) -> H264InitReflistDb11R {
        H264InitReflistDb11R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 6st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_db6(&mut self) -> H264InitReflistDb6W<Swreg104Spec> {
        H264InitReflistDb6W::new(self, 0)
    }
    #[doc = "Bits 5:9 - 7st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_db7(&mut self) -> H264InitReflistDb7W<Swreg104Spec> {
        H264InitReflistDb7W::new(self, 5)
    }
    #[doc = "Bits 10:14 - 8st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_db8(&mut self) -> H264InitReflistDb8W<Swreg104Spec> {
        H264InitReflistDb8W::new(self, 10)
    }
    #[doc = "Bits 15:19 - 9st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_db9(&mut self) -> H264InitReflistDb9W<Swreg104Spec> {
        H264InitReflistDb9W::new(self, 15)
    }
    #[doc = "Bits 20:24 - 10st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_db10(&mut self) -> H264InitReflistDb10W<Swreg104Spec> {
        H264InitReflistDb10W::new(self, 20)
    }
    #[doc = "Bits 25:29 - 11st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_db11(&mut self) -> H264InitReflistDb11W<Swreg104Spec> {
        H264InitReflistDb11W::new(self, 25)
    }
}
#[doc = "initial reference picture list related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg104::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg104::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg104Spec;
impl crate::RegisterSpec for Swreg104Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg104::R`](R) reader structure"]
impl crate::Readable for Swreg104Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg104::W`](W) writer structure"]
impl crate::Writable for Swreg104Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG104 to value 0"]
impl crate::Resettable for Swreg104Spec {
    const RESET_VALUE: u32 = 0;
}
