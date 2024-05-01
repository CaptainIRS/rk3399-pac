#[doc = "Register `SWREG103` reader"]
pub type R = crate::R<Swreg103Spec>;
#[doc = "Register `SWREG103` writer"]
pub type W = crate::W<Swreg103Spec>;
#[doc = "Field `H264_INIT_REFLIST_DB0` reader - 0st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb0R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DB0` writer - 0st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DB1` reader - 1st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb1R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DB1` writer - 1st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DB2` reader - 2st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb2R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DB2` writer - 2st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DB3` reader - 3st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb3R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DB3` writer - 3st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DB4` reader - 4st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb4R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DB4` writer - 4st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DB5` reader - 5st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb5R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DB5` writer - 5st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 0st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_db0(&self) -> H264InitReflistDb0R {
        H264InitReflistDb0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 1st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_db1(&self) -> H264InitReflistDb1R {
        H264InitReflistDb1R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 2st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_db2(&self) -> H264InitReflistDb2R {
        H264InitReflistDb2R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 3st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_db3(&self) -> H264InitReflistDb3R {
        H264InitReflistDb3R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 4st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_db4(&self) -> H264InitReflistDb4R {
        H264InitReflistDb4R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - 5st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_db5(&self) -> H264InitReflistDb5R {
        H264InitReflistDb5R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 0st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_db0(&mut self) -> H264InitReflistDb0W<Swreg103Spec> {
        H264InitReflistDb0W::new(self, 0)
    }
    #[doc = "Bits 5:9 - 1st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_db1(&mut self) -> H264InitReflistDb1W<Swreg103Spec> {
        H264InitReflistDb1W::new(self, 5)
    }
    #[doc = "Bits 10:14 - 2st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_db2(&mut self) -> H264InitReflistDb2W<Swreg103Spec> {
        H264InitReflistDb2W::new(self, 10)
    }
    #[doc = "Bits 15:19 - 3st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_db3(&mut self) -> H264InitReflistDb3W<Swreg103Spec> {
        H264InitReflistDb3W::new(self, 15)
    }
    #[doc = "Bits 20:24 - 4st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_db4(&mut self) -> H264InitReflistDb4W<Swreg103Spec> {
        H264InitReflistDb4W::new(self, 20)
    }
    #[doc = "Bits 25:29 - 5st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_db5(&mut self) -> H264InitReflistDb5W<Swreg103Spec> {
        H264InitReflistDb5W::new(self, 25)
    }
}
#[doc = "initial reference picture list related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg103::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg103::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg103Spec;
impl crate::RegisterSpec for Swreg103Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg103::R`](R) reader structure"]
impl crate::Readable for Swreg103Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg103::W`](W) writer structure"]
impl crate::Writable for Swreg103Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG103 to value 0"]
impl crate::Resettable for Swreg103Spec {
    const RESET_VALUE: u32 = 0;
}
