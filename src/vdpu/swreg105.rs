#[doc = "Register `SWREG105` reader"]
pub type R = crate::R<Swreg105Spec>;
#[doc = "Register `SWREG105` writer"]
pub type W = crate::W<Swreg105Spec>;
#[doc = "Field `H264_INIT_REFLIST_DB12` reader - 12st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb12R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DB12` writer - 12st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb12W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DB13` reader - 13st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb13R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DB13` writer - 13st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb13W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DB14` reader - 14st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb14R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DB14` writer - 14st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb14W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_DB15` reader - 15st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb15R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_DB15` writer - 15st initial reference picture list for direct backward picid\n\nused for h264"]
pub type H264InitReflistDb15W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 12st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_db12(&self) -> H264InitReflistDb12R {
        H264InitReflistDb12R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 13st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_db13(&self) -> H264InitReflistDb13R {
        H264InitReflistDb13R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 14st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_db14(&self) -> H264InitReflistDb14R {
        H264InitReflistDb14R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 15st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    pub fn h264_init_reflist_db15(&self) -> H264InitReflistDb15R {
        H264InitReflistDb15R::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 12st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_db12(&mut self) -> H264InitReflistDb12W<Swreg105Spec> {
        H264InitReflistDb12W::new(self, 0)
    }
    #[doc = "Bits 5:9 - 13st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_db13(&mut self) -> H264InitReflistDb13W<Swreg105Spec> {
        H264InitReflistDb13W::new(self, 5)
    }
    #[doc = "Bits 10:14 - 14st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_db14(&mut self) -> H264InitReflistDb14W<Swreg105Spec> {
        H264InitReflistDb14W::new(self, 10)
    }
    #[doc = "Bits 15:19 - 15st initial reference picture list for direct backward picid\n\nused for h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_db15(&mut self) -> H264InitReflistDb15W<Swreg105Spec> {
        H264InitReflistDb15W::new(self, 15)
    }
}
#[doc = "initial reference picture list related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg105::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg105::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg105Spec;
impl crate::RegisterSpec for Swreg105Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg105::R`](R) reader structure"]
impl crate::Readable for Swreg105Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg105::W`](W) writer structure"]
impl crate::Writable for Swreg105Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG105 to value 0"]
impl crate::Resettable for Swreg105Spec {
    const RESET_VALUE: u32 = 0;
}
