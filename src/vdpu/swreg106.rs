#[doc = "Register `SWREG106` reader"]
pub type R = crate::R<Swreg106Spec>;
#[doc = "Register `SWREG106` writer"]
pub type W = crate::W<Swreg106Spec>;
#[doc = "Field `H264_INIT_REFLIST_PF0` reader - 0st initial reference picture list for P forward picid\n\nused in 264"]
pub type H264InitReflistPf0R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_PF0` writer - 0st initial reference picture list for P forward picid\n\nused in 264"]
pub type H264InitReflistPf0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_PF1` reader - 1st initial reference picture list for P forward picid\n\nInitial reference picture list for P forward picid 1"]
pub type H264InitReflistPf1R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_PF1` writer - 1st initial reference picture list for P forward picid\n\nInitial reference picture list for P forward picid 1"]
pub type H264InitReflistPf1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_PF2` reader - 2st initial reference picture list for P forward picid\n\nInitial reference picture list for P forward picid 2"]
pub type H264InitReflistPf2R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_PF2` writer - 2st initial reference picture list for P forward picid\n\nInitial reference picture list for P forward picid 2"]
pub type H264InitReflistPf2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_INIT_REFLIST_PF3` reader - 3st initial reference picture list for P forward picid\n\nInitial reference picture list for P forward picid 3"]
pub type H264InitReflistPf3R = crate::FieldReader;
#[doc = "Field `H264_INIT_REFLIST_PF3` writer - 3st initial reference picture list for P forward picid\n\nInitial reference picture list for P forward picid 3"]
pub type H264InitReflistPf3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 0st initial reference picture list for P forward picid\n\nused in 264"]
    #[inline(always)]
    pub fn h264_init_reflist_pf0(&self) -> H264InitReflistPf0R {
        H264InitReflistPf0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 1st initial reference picture list for P forward picid\n\nInitial reference picture list for P forward picid 1"]
    #[inline(always)]
    pub fn h264_init_reflist_pf1(&self) -> H264InitReflistPf1R {
        H264InitReflistPf1R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 2st initial reference picture list for P forward picid\n\nInitial reference picture list for P forward picid 2"]
    #[inline(always)]
    pub fn h264_init_reflist_pf2(&self) -> H264InitReflistPf2R {
        H264InitReflistPf2R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 3st initial reference picture list for P forward picid\n\nInitial reference picture list for P forward picid 3"]
    #[inline(always)]
    pub fn h264_init_reflist_pf3(&self) -> H264InitReflistPf3R {
        H264InitReflistPf3R::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 0st initial reference picture list for P forward picid\n\nused in 264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_pf0(&mut self) -> H264InitReflistPf0W<Swreg106Spec> {
        H264InitReflistPf0W::new(self, 0)
    }
    #[doc = "Bits 5:9 - 1st initial reference picture list for P forward picid\n\nInitial reference picture list for P forward picid 1"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_pf1(&mut self) -> H264InitReflistPf1W<Swreg106Spec> {
        H264InitReflistPf1W::new(self, 5)
    }
    #[doc = "Bits 10:14 - 2st initial reference picture list for P forward picid\n\nInitial reference picture list for P forward picid 2"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_pf2(&mut self) -> H264InitReflistPf2W<Swreg106Spec> {
        H264InitReflistPf2W::new(self, 10)
    }
    #[doc = "Bits 15:19 - 3st initial reference picture list for P forward picid\n\nInitial reference picture list for P forward picid 3"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_reflist_pf3(&mut self) -> H264InitReflistPf3W<Swreg106Spec> {
        H264InitReflistPf3W::new(self, 15)
    }
}
#[doc = "initial reference picture list related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg106::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg106::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg106Spec;
impl crate::RegisterSpec for Swreg106Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg106::R`](R) reader structure"]
impl crate::Readable for Swreg106Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg106::W`](W) writer structure"]
impl crate::Writable for Swreg106Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG106 to value 0"]
impl crate::Resettable for Swreg106Spec {
    const RESET_VALUE: u32 = 0;
}
