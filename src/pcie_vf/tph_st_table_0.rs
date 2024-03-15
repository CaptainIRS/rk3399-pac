#[doc = "Register `TPH_ST_TABLE_0` reader"]
pub type R = crate::R<TphStTable0Spec>;
#[doc = "Register `TPH_ST_TABLE_0` writer"]
pub type W = crate::W<TphStTable0Spec>;
#[doc = "Field `ST0L` reader - ST 0 Lower \\[ST0L\\]
Lower 8 bits of the first Steering Tag. This is the 8- bit Steering Tag sent out in requests."]
pub type St0lR = crate::FieldReader;
#[doc = "Field `ST0L` writer - ST 0 Lower \\[ST0L\\]
Lower 8 bits of the first Steering Tag. This is the 8- bit Steering Tag sent out in requests."]
pub type St0lW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ST0U` reader - ST 0 Upper \\[ST0U\\]
This field is used for the upper 8 bits of the first Steering Tag when Extended TPH Requester support is enabled."]
pub type St0uR = crate::FieldReader;
#[doc = "Field `ST1L` reader - ST 1 Lower \\[ST1L\\]
Lower 8 bits of the second Steering Tag. This is the 8-bit Steering Tag sent out in requests."]
pub type St1lR = crate::FieldReader;
#[doc = "Field `ST1L` writer - ST 1 Lower \\[ST1L\\]
Lower 8 bits of the second Steering Tag. This is the 8-bit Steering Tag sent out in requests."]
pub type St1lW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ST1U` reader - ST 1 Upper \\[ST1U\\]
This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
pub type St1uR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - ST 0 Lower \\[ST0L\\]
Lower 8 bits of the first Steering Tag. This is the 8- bit Steering Tag sent out in requests."]
    #[inline(always)]
    pub fn st0l(&self) -> St0lR {
        St0lR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ST 0 Upper \\[ST0U\\]
This field is used for the upper 8 bits of the first Steering Tag when Extended TPH Requester support is enabled."]
    #[inline(always)]
    pub fn st0u(&self) -> St0uR {
        St0uR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ST 1 Lower \\[ST1L\\]
Lower 8 bits of the second Steering Tag. This is the 8-bit Steering Tag sent out in requests."]
    #[inline(always)]
    pub fn st1l(&self) -> St1lR {
        St1lR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ST 1 Upper \\[ST1U\\]
This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
    #[inline(always)]
    pub fn st1u(&self) -> St1uR {
        St1uR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ST 0 Lower \\[ST0L\\]
Lower 8 bits of the first Steering Tag. This is the 8- bit Steering Tag sent out in requests."]
    #[inline(always)]
    #[must_use]
    pub fn st0l(&mut self) -> St0lW<TphStTable0Spec> {
        St0lW::new(self, 0)
    }
    #[doc = "Bits 16:23 - ST 1 Lower \\[ST1L\\]
Lower 8 bits of the second Steering Tag. This is the 8-bit Steering Tag sent out in requests."]
    #[inline(always)]
    #[must_use]
    pub fn st1l(&mut self) -> St1lW<TphStTable0Spec> {
        St1lW::new(self, 16)
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
