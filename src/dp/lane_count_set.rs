#[doc = "Register `LANE_COUNT_SET` reader"]
pub type R = crate::R<LaneCountSetSpec>;
#[doc = "Register `LANE_COUNT_SET` writer"]
pub type W = crate::W<LaneCountSetSpec>;
#[doc = "Main link lane count\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LaneCountSet {
    #[doc = "1: four lanes other: Reserved"]
    H1 = 1,
    #[doc = "2: four lanes other: Reserved"]
    H2 = 2,
    #[doc = "4: four lanes other: Reserved"]
    H4 = 4,
}
impl From<LaneCountSet> for u8 {
    #[inline(always)]
    fn from(variant: LaneCountSet) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LaneCountSet {
    type Ux = u8;
}
#[doc = "Field `LANE_COUNT_SET` reader - Main link lane count"]
pub type LaneCountSetR = crate::FieldReader<LaneCountSet>;
impl LaneCountSetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LaneCountSet> {
        match self.bits {
            1 => Some(LaneCountSet::H1),
            2 => Some(LaneCountSet::H2),
            4 => Some(LaneCountSet::H4),
            _ => None,
        }
    }
    #[doc = "four lanes other: Reserved"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == LaneCountSet::H1
    }
    #[doc = "four lanes other: Reserved"]
    #[inline(always)]
    pub fn is_h2(&self) -> bool {
        *self == LaneCountSet::H2
    }
    #[doc = "four lanes other: Reserved"]
    #[inline(always)]
    pub fn is_h4(&self) -> bool {
        *self == LaneCountSet::H4
    }
}
#[doc = "Field `LANE_COUNT_SET` writer - Main link lane count"]
pub type LaneCountSetW<'a, REG> = crate::FieldWriter<'a, REG, 3, LaneCountSet>;
impl<'a, REG> LaneCountSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "four lanes other: Reserved"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(LaneCountSet::H1)
    }
    #[doc = "four lanes other: Reserved"]
    #[inline(always)]
    pub fn h2(self) -> &'a mut crate::W<REG> {
        self.variant(LaneCountSet::H2)
    }
    #[doc = "four lanes other: Reserved"]
    #[inline(always)]
    pub fn h4(self) -> &'a mut crate::W<REG> {
        self.variant(LaneCountSet::H4)
    }
}
impl R {
    #[doc = "Bits 0:2 - Main link lane count"]
    #[inline(always)]
    pub fn lane_count_set(&self) -> LaneCountSetR {
        LaneCountSetR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Main link lane count"]
    #[inline(always)]
    #[must_use]
    pub fn lane_count_set(&mut self) -> LaneCountSetW<LaneCountSetSpec> {
        LaneCountSetW::new(self, 0)
    }
}
#[doc = "DP Main Link Lane Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lane_count_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lane_count_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LaneCountSetSpec;
impl crate::RegisterSpec for LaneCountSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lane_count_set::R`](R) reader structure"]
impl crate::Readable for LaneCountSetSpec {}
#[doc = "`write(|w| ..)` method takes [`lane_count_set::W`](W) writer structure"]
impl crate::Writable for LaneCountSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07;
}
#[doc = "`reset()` method sets LANE_COUNT_SET to value 0x04"]
impl crate::Resettable for LaneCountSetSpec {
    const RESET_VALUE: u32 = 0x04;
}
