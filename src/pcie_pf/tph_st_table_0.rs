#[doc = "Register `TPH_ST_TABLE_0` reader"]
pub type R = crate::R<TphStTable0Spec>;
#[doc = "Register `TPH_ST_TABLE_0` writer"]
pub type W = crate::W<TphStTable0Spec>;
#[doc = "Field `STL0` reader - ST Lower 0 \\[STL0\\]
Lower 8 bits of the first Steering Tag. This is the 8- bit Steering Tag sent out in requests."]
pub type Stl0R = crate::FieldReader;
#[doc = "Field `STL0` writer - ST Lower 0 \\[STL0\\]
Lower 8 bits of the first Steering Tag. This is the 8- bit Steering Tag sent out in requests."]
pub type Stl0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `STU0` reader - ST Upper 0 \\[STU0\\]
This field is used for the upper 8 bits of the first Steering Tag when Extended TPH Requester support is enabled."]
pub type Stu0R = crate::FieldReader;
#[doc = "Field `STL1` reader - ST Lower 1 \\[STL1\\]
Lower 8 bits of the second Steering Tag. This is the 8-bit Steering Tag sent out in requests."]
pub type Stl1R = crate::FieldReader;
#[doc = "Field `STL1` writer - ST Lower 1 \\[STL1\\]
Lower 8 bits of the second Steering Tag. This is the 8-bit Steering Tag sent out in requests."]
pub type Stl1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `STU1` reader - ST Upper 1 \\[STU1\\]
This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
pub type Stu1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - ST Lower 0 \\[STL0\\]
Lower 8 bits of the first Steering Tag. This is the 8- bit Steering Tag sent out in requests."]
    #[inline(always)]
    pub fn stl0(&self) -> Stl0R {
        Stl0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ST Upper 0 \\[STU0\\]
This field is used for the upper 8 bits of the first Steering Tag when Extended TPH Requester support is enabled."]
    #[inline(always)]
    pub fn stu0(&self) -> Stu0R {
        Stu0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ST Lower 1 \\[STL1\\]
Lower 8 bits of the second Steering Tag. This is the 8-bit Steering Tag sent out in requests."]
    #[inline(always)]
    pub fn stl1(&self) -> Stl1R {
        Stl1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ST Upper 1 \\[STU1\\]
This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
    #[inline(always)]
    pub fn stu1(&self) -> Stu1R {
        Stu1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ST Lower 0 \\[STL0\\]
Lower 8 bits of the first Steering Tag. This is the 8- bit Steering Tag sent out in requests."]
    #[inline(always)]
    #[must_use]
    pub fn stl0(&mut self) -> Stl0W<TphStTable0Spec> {
        Stl0W::new(self, 0)
    }
    #[doc = "Bits 16:23 - ST Lower 1 \\[STL1\\]
Lower 8 bits of the second Steering Tag. This is the 8-bit Steering Tag sent out in requests."]
    #[inline(always)]
    #[must_use]
    pub fn stl1(&mut self) -> Stl1W<TphStTable0Spec> {
        Stl1W::new(self, 16)
    }
}
#[doc = "TPH ST Table 0 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tph_st_table_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tph_st_table_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TphStTable0Spec;
impl crate::RegisterSpec for TphStTable0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tph_st_table_0::R`](R) reader structure"]
impl crate::Readable for TphStTable0Spec {}
#[doc = "`write(|w| ..)` method takes [`tph_st_table_0::W`](W) writer structure"]
impl crate::Writable for TphStTable0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TPH_ST_TABLE_0 to value 0"]
impl crate::Resettable for TphStTable0Spec {
    const RESET_VALUE: u32 = 0;
}
