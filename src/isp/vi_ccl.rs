#[doc = "Register `VI_CCL` reader"]
pub type R = crate::R<ViCclSpec>;
#[doc = "Register `VI_CCL` writer"]
pub type W = crate::W<ViCclSpec>;
#[doc = "Field `vi_ccl_dis_status` reader - Status of vi_ccl\\[2\\]
bit (copy of vi_ccl\\[2\\])"]
pub type ViCclDisStatusR = crate::BitReader;
#[doc = "Field `vi_ccl_dis_status` writer - Status of vi_ccl\\[2\\]
bit (copy of vi_ccl\\[2\\])"]
pub type ViCclDisStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock Control Logic disable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViCclDis {
    #[doc = "0: processing/cfg-clocks for all ISP sub modules enabled"]
    B0 = 0,
    #[doc = "1: processing/cfg-clocks for all ISP sub modules disabled w/o access to ID and VI_CCL register"]
    B1 = 1,
}
impl From<ViCclDis> for bool {
    #[inline(always)]
    fn from(variant: ViCclDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_ccl_dis` reader - Clock Control Logic disable"]
pub type ViCclDisR = crate::BitReader<ViCclDis>;
impl ViCclDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViCclDis {
        match self.bits {
            false => ViCclDis::B0,
            true => ViCclDis::B1,
        }
    }
    #[doc = "processing/cfg-clocks for all ISP sub modules enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViCclDis::B0
    }
    #[doc = "processing/cfg-clocks for all ISP sub modules disabled w/o access to ID and VI_CCL register"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViCclDis::B1
    }
}
#[doc = "Field `vi_ccl_dis` writer - Clock Control Logic disable"]
pub type ViCclDisW<'a, REG> = crate::BitWriter<'a, REG, ViCclDis>;
impl<'a, REG> ViCclDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing/cfg-clocks for all ISP sub modules enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViCclDis::B0)
    }
    #[doc = "processing/cfg-clocks for all ISP sub modules disabled w/o access to ID and VI_CCL register"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViCclDis::B1)
    }
}
impl R {
    #[doc = "Bit 1 - Status of vi_ccl\\[2\\]
bit (copy of vi_ccl\\[2\\])"]
    #[inline(always)]
    pub fn vi_ccl_dis_status(&self) -> ViCclDisStatusR {
        ViCclDisStatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Control Logic disable"]
    #[inline(always)]
    pub fn vi_ccl_dis(&self) -> ViCclDisR {
        ViCclDisR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Status of vi_ccl\\[2\\]
bit (copy of vi_ccl\\[2\\])"]
    #[inline(always)]
    #[must_use]
    pub fn vi_ccl_dis_status(&mut self) -> ViCclDisStatusW<ViCclSpec> {
        ViCclDisStatusW::new(self, 1)
    }
    #[doc = "Bit 2 - Clock Control Logic disable"]
    #[inline(always)]
    #[must_use]
    pub fn vi_ccl_dis(&mut self) -> ViCclDisW<ViCclSpec> {
        ViCclDisW::new(self, 2)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vi_ccl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vi_ccl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ViCclSpec;
impl crate::RegisterSpec for ViCclSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vi_ccl::R`](R) reader structure"]
impl crate::Readable for ViCclSpec {}
#[doc = "`write(|w| ..)` method takes [`vi_ccl::W`](W) writer structure"]
impl crate::Writable for ViCclSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VI_CCL to value 0"]
impl crate::Resettable for ViCclSpec {
    const RESET_VALUE: u32 = 0;
}
