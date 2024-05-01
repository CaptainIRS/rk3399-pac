#[doc = "Register `PAT_CON` reader"]
pub type R = crate::R<PatConSpec>;
#[doc = "Register `PAT_CON` writer"]
pub type W = crate::W<PatConSpec>;
#[doc = "Field `SW_PAT_WIDTH` reader - Pattern width"]
pub type SwPatWidthR = crate::FieldReader;
#[doc = "Field `SW_PAT_WIDTH` writer - Pattern width"]
pub type SwPatWidthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_PAT_HEIGHT` reader - Pattern height"]
pub type SwPatHeightR = crate::FieldReader;
#[doc = "Field `SW_PAT_HEIGHT` writer - Pattern height"]
pub type SwPatHeightW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_PAT_OFFSET_X` reader - Pattern x offset"]
pub type SwPatOffsetXR = crate::FieldReader;
#[doc = "Field `SW_PAT_OFFSET_X` writer - Pattern x offset"]
pub type SwPatOffsetXW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_PAT_OFFSET_Y` reader - Pattern y offset"]
pub type SwPatOffsetYR = crate::FieldReader;
#[doc = "Field `SW_PAT_OFFSET_Y` writer - Pattern y offset"]
pub type SwPatOffsetYW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Pattern width"]
    #[inline(always)]
    pub fn sw_pat_width(&self) -> SwPatWidthR {
        SwPatWidthR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Pattern height"]
    #[inline(always)]
    pub fn sw_pat_height(&self) -> SwPatHeightR {
        SwPatHeightR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Pattern x offset"]
    #[inline(always)]
    pub fn sw_pat_offset_x(&self) -> SwPatOffsetXR {
        SwPatOffsetXR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Pattern y offset"]
    #[inline(always)]
    pub fn sw_pat_offset_y(&self) -> SwPatOffsetYR {
        SwPatOffsetYR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pattern width"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pat_width(&mut self) -> SwPatWidthW<PatConSpec> {
        SwPatWidthW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Pattern height"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pat_height(&mut self) -> SwPatHeightW<PatConSpec> {
        SwPatHeightW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Pattern x offset"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pat_offset_x(&mut self) -> SwPatOffsetXW<PatConSpec> {
        SwPatOffsetXW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Pattern y offset"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pat_offset_y(&mut self) -> SwPatOffsetYW<PatConSpec> {
        SwPatOffsetYW::new(self, 24)
    }
}
#[doc = "Pattern size/offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pat_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pat_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PatConSpec;
impl crate::RegisterSpec for PatConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pat_con::R`](R) reader structure"]
impl crate::Readable for PatConSpec {}
#[doc = "`write(|w| ..)` method takes [`pat_con::W`](W) writer structure"]
impl crate::Writable for PatConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAT_CON to value 0"]
impl crate::Resettable for PatConSpec {
    const RESET_VALUE: u32 = 0;
}
