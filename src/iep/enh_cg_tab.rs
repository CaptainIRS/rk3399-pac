#[doc = "Register `ENH_CG_TAB` reader"]
pub type R = crate::R<EnhCgTabSpec>;
#[doc = "Register `ENH_CG_TAB` writer"]
pub type W = crate::W<EnhCgTabSpec>;
#[doc = "Field `CG_TAB_0` reader - cg table 0\n\n256x8bit contrast &amp; gamma mapping table\n\npixel value 0,4,8,12,......mapping"]
pub type CgTab0R = crate::FieldReader;
#[doc = "Field `CG_TAB_0` writer - cg table 0\n\n256x8bit contrast &amp; gamma mapping table\n\npixel value 0,4,8,12,......mapping"]
pub type CgTab0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CG_TAB_1` reader - cg table 1\n\npixel value 1,5,9,13,......mapping"]
pub type CgTab1R = crate::FieldReader;
#[doc = "Field `CG_TAB_1` writer - cg table 1\n\npixel value 1,5,9,13,......mapping"]
pub type CgTab1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CG_TAB_2` reader - cg table 2\n\npixel value 2,6,10,14,......mapping"]
pub type CgTab2R = crate::FieldReader;
#[doc = "Field `CG_TAB_2` writer - cg table 2\n\npixel value 2,6,10,14,......mapping"]
pub type CgTab2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CG_TAB_3` reader - cg table 3\n\npixel value 3,7,11,15,......mapping"]
pub type CgTab3R = crate::FieldReader;
#[doc = "Field `CG_TAB_3` writer - cg table 3\n\npixel value 3,7,11,15,......mapping"]
pub type CgTab3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - cg table 0\n\n256x8bit contrast &amp; gamma mapping table\n\npixel value 0,4,8,12,......mapping"]
    #[inline(always)]
    pub fn cg_tab_0(&self) -> CgTab0R {
        CgTab0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - cg table 1\n\npixel value 1,5,9,13,......mapping"]
    #[inline(always)]
    pub fn cg_tab_1(&self) -> CgTab1R {
        CgTab1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - cg table 2\n\npixel value 2,6,10,14,......mapping"]
    #[inline(always)]
    pub fn cg_tab_2(&self) -> CgTab2R {
        CgTab2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - cg table 3\n\npixel value 3,7,11,15,......mapping"]
    #[inline(always)]
    pub fn cg_tab_3(&self) -> CgTab3R {
        CgTab3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - cg table 0\n\n256x8bit contrast &amp; gamma mapping table\n\npixel value 0,4,8,12,......mapping"]
    #[inline(always)]
    #[must_use]
    pub fn cg_tab_0(&mut self) -> CgTab0W<EnhCgTabSpec> {
        CgTab0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - cg table 1\n\npixel value 1,5,9,13,......mapping"]
    #[inline(always)]
    #[must_use]
    pub fn cg_tab_1(&mut self) -> CgTab1W<EnhCgTabSpec> {
        CgTab1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - cg table 2\n\npixel value 2,6,10,14,......mapping"]
    #[inline(always)]
    #[must_use]
    pub fn cg_tab_2(&mut self) -> CgTab2W<EnhCgTabSpec> {
        CgTab2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - cg table 3\n\npixel value 3,7,11,15,......mapping"]
    #[inline(always)]
    #[must_use]
    pub fn cg_tab_3(&mut self) -> CgTab3W<EnhCgTabSpec> {
        CgTab3W::new(self, 24)
    }
}
#[doc = "contrast and gamma enhancement table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enh_cg_tab::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enh_cg_tab::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnhCgTabSpec;
impl crate::RegisterSpec for EnhCgTabSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enh_cg_tab::R`](R) reader structure"]
impl crate::Readable for EnhCgTabSpec {}
#[doc = "`write(|w| ..)` method takes [`enh_cg_tab::W`](W) writer structure"]
impl crate::Writable for EnhCgTabSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENH_CG_TAB to value 0"]
impl crate::Resettable for EnhCgTabSpec {
    const RESET_VALUE: u32 = 0;
}
