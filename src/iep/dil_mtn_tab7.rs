#[doc = "Register `DIL_MTN_TAB7` reader"]
pub type R = crate::R<DilMtnTab7Spec>;
#[doc = "Register `DIL_MTN_TAB7` writer"]
pub type W = crate::W<DilMtnTab7Spec>;
#[doc = "Field `MTN_SUB_TAB0` reader - motion sub table0"]
pub type MtnSubTab0R = crate::FieldReader;
#[doc = "Field `MTN_SUB_TAB0` writer - motion sub table0"]
pub type MtnSubTab0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MTN_SUB_TAB1` reader - motion sub table1"]
pub type MtnSubTab1R = crate::FieldReader;
#[doc = "Field `MTN_SUB_TAB1` writer - motion sub table1"]
pub type MtnSubTab1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MTN_SUB_TAB2` reader - motion sub table2"]
pub type MtnSubTab2R = crate::FieldReader;
#[doc = "Field `MTN_SUB_TAB2` writer - motion sub table2"]
pub type MtnSubTab2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MTN_SUB_TAB3` reader - motion sub table3"]
pub type MtnSubTab3R = crate::FieldReader;
#[doc = "Field `MTN_SUB_TAB3` writer - motion sub table3"]
pub type MtnSubTab3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - motion sub table0"]
    #[inline(always)]
    pub fn mtn_sub_tab0(&self) -> MtnSubTab0R {
        MtnSubTab0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - motion sub table1"]
    #[inline(always)]
    pub fn mtn_sub_tab1(&self) -> MtnSubTab1R {
        MtnSubTab1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - motion sub table2"]
    #[inline(always)]
    pub fn mtn_sub_tab2(&self) -> MtnSubTab2R {
        MtnSubTab2R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - motion sub table3"]
    #[inline(always)]
    pub fn mtn_sub_tab3(&self) -> MtnSubTab3R {
        MtnSubTab3R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - motion sub table0"]
    #[inline(always)]
    #[must_use]
    pub fn mtn_sub_tab0(&mut self) -> MtnSubTab0W<DilMtnTab7Spec> {
        MtnSubTab0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - motion sub table1"]
    #[inline(always)]
    #[must_use]
    pub fn mtn_sub_tab1(&mut self) -> MtnSubTab1W<DilMtnTab7Spec> {
        MtnSubTab1W::new(self, 8)
    }
    #[doc = "Bits 16:22 - motion sub table2"]
    #[inline(always)]
    #[must_use]
    pub fn mtn_sub_tab2(&mut self) -> MtnSubTab2W<DilMtnTab7Spec> {
        MtnSubTab2W::new(self, 16)
    }
    #[doc = "Bits 24:30 - motion sub table3"]
    #[inline(always)]
    #[must_use]
    pub fn mtn_sub_tab3(&mut self) -> MtnSubTab3W<DilMtnTab7Spec> {
        MtnSubTab3W::new(self, 24)
    }
}
#[doc = "Deinterlace motion table7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dil_mtn_tab7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dil_mtn_tab7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DilMtnTab7Spec;
impl crate::RegisterSpec for DilMtnTab7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dil_mtn_tab7::R`](R) reader structure"]
impl crate::Readable for DilMtnTab7Spec {}
#[doc = "`write(|w| ..)` method takes [`dil_mtn_tab7::W`](W) writer structure"]
impl crate::Writable for DilMtnTab7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIL_MTN_TAB7 to value 0"]
impl crate::Resettable for DilMtnTab7Spec {
    const RESET_VALUE: u32 = 0;
}
