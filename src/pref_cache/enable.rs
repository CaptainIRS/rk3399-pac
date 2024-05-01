#[doc = "Register `ENABLE` reader"]
pub type R = crate::R<EnableSpec>;
#[doc = "Register `ENABLE` writer"]
pub type W = crate::W<EnableSpec>;
#[doc = "Field `PERMIT_CACHEABLE_ACCESS` reader - cacheable access\n\n1'b1: permit cacheable access"]
pub type PermitCacheableAccessR = crate::BitReader;
#[doc = "Field `PERMIT_CACHEABLE_ACCESS` writer - cacheable access\n\n1'b1: permit cacheable access"]
pub type PermitCacheableAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERMIT_CACHE_READ_ALLOCATE` reader - cache read allocate\n\n1'b1: permit cache read allocate"]
pub type PermitCacheReadAllocateR = crate::BitReader;
#[doc = "Field `PERMIT_CACHE_READ_ALLOCATE` writer - cache read allocate\n\n1'b1: permit cache read allocate"]
pub type PermitCacheReadAllocateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_READBUFFER_COUNTER_REJECT_EN` reader - counter reject enable\n\ndefault is 1'b0, for enhance cacheable read performnace in\n\nreadbuffer.\n\n1'b1: normal origin counter reject"]
pub type SwReadbufferCounterRejectEnR = crate::BitReader;
#[doc = "Field `SW_READBUFFER_COUNTER_REJECT_EN` writer - counter reject enable\n\ndefault is 1'b0, for enhance cacheable read performnace in\n\nreadbuffer.\n\n1'b1: normal origin counter reject"]
pub type SwReadbufferCounterRejectEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_CACHE_CLK_DISGATE` reader - cache clk disgate\n\ncache clk disgate\n\nwhen it is 1'b0, enable cache clk auto clkgating\n\nwhen it is 1'b1, disable cache clk auto clkgating"]
pub type SwCacheClkDisgateR = crate::BitReader;
#[doc = "Field `SW_CACHE_CLK_DISGATE` writer - cache clk disgate\n\ncache clk disgate\n\nwhen it is 1'b0, enable cache clk auto clkgating\n\nwhen it is 1'b1, disable cache clk auto clkgating"]
pub type SwCacheClkDisgateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - cacheable access\n\n1'b1: permit cacheable access"]
    #[inline(always)]
    pub fn permit_cacheable_access(&self) -> PermitCacheableAccessR {
        PermitCacheableAccessR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - cache read allocate\n\n1'b1: permit cache read allocate"]
    #[inline(always)]
    pub fn permit_cache_read_allocate(&self) -> PermitCacheReadAllocateR {
        PermitCacheReadAllocateR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - counter reject enable\n\ndefault is 1'b0, for enhance cacheable read performnace in\n\nreadbuffer.\n\n1'b1: normal origin counter reject"]
    #[inline(always)]
    pub fn sw_readbuffer_counter_reject_en(&self) -> SwReadbufferCounterRejectEnR {
        SwReadbufferCounterRejectEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - cache clk disgate\n\ncache clk disgate\n\nwhen it is 1'b0, enable cache clk auto clkgating\n\nwhen it is 1'b1, disable cache clk auto clkgating"]
    #[inline(always)]
    pub fn sw_cache_clk_disgate(&self) -> SwCacheClkDisgateR {
        SwCacheClkDisgateR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - cacheable access\n\n1'b1: permit cacheable access"]
    #[inline(always)]
    #[must_use]
    pub fn permit_cacheable_access(&mut self) -> PermitCacheableAccessW<EnableSpec> {
        PermitCacheableAccessW::new(self, 0)
    }
    #[doc = "Bit 1 - cache read allocate\n\n1'b1: permit cache read allocate"]
    #[inline(always)]
    #[must_use]
    pub fn permit_cache_read_allocate(&mut self) -> PermitCacheReadAllocateW<EnableSpec> {
        PermitCacheReadAllocateW::new(self, 1)
    }
    #[doc = "Bit 2 - counter reject enable\n\ndefault is 1'b0, for enhance cacheable read performnace in\n\nreadbuffer.\n\n1'b1: normal origin counter reject"]
    #[inline(always)]
    #[must_use]
    pub fn sw_readbuffer_counter_reject_en(&mut self) -> SwReadbufferCounterRejectEnW<EnableSpec> {
        SwReadbufferCounterRejectEnW::new(self, 2)
    }
    #[doc = "Bit 3 - cache clk disgate\n\ncache clk disgate\n\nwhen it is 1'b0, enable cache clk auto clkgating\n\nwhen it is 1'b1, disable cache clk auto clkgating"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cache_clk_disgate(&mut self) -> SwCacheClkDisgateW<EnableSpec> {
        SwCacheClkDisgateW::new(self, 3)
    }
}
#[doc = "enables cacheable accesses and cache read allocation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableSpec;
impl crate::RegisterSpec for EnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable::R`](R) reader structure"]
impl crate::Readable for EnableSpec {}
#[doc = "`write(|w| ..)` method takes [`enable::W`](W) writer structure"]
impl crate::Writable for EnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENABLE to value 0x03"]
impl crate::Resettable for EnableSpec {
    const RESET_VALUE: u32 = 0x03;
}
