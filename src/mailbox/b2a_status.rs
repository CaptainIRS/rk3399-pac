#[doc = "Register `B2A_STATUS` reader"]
pub type R = crate::R<B2aStatusSpec>;
#[doc = "Register `B2A_STATUS` writer"]
pub type W = crate::W<B2aStatusSpec>;
#[doc = "Field `INT0` reader - interrupt status for int0.\n\nwhen writte 1, int is cleared."]
pub type Int0R = crate::BitReader;
#[doc = "Field `INT0` writer - interrupt status for int0.\n\nwhen writte 1, int is cleared."]
pub type Int0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1` reader - interrupt status for int1.\n\nwhen writte 1, int is cleared."]
pub type Int1R = crate::BitReader;
#[doc = "Field `INT1` writer - interrupt status for int1.\n\nwhen writte 1, int is cleared."]
pub type Int1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2` reader - interrupt status for int2.\n\nwhen writte 1, int is cleared."]
pub type Int2R = crate::BitReader;
#[doc = "Field `INT2` writer - interrupt status for int2.\n\nwhen writte 1, int is cleared."]
pub type Int2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT3` reader - interrupt status for int3.\n\nwhen writte 1, int is cleared."]
pub type Int3R = crate::BitReader;
#[doc = "Field `INT3` writer - interrupt status for int3.\n\nwhen writte 1, int is cleared."]
pub type Int3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - interrupt status for int0.\n\nwhen writte 1, int is cleared."]
    #[inline(always)]
    pub fn int0(&self) -> Int0R {
        Int0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interrupt status for int1.\n\nwhen writte 1, int is cleared."]
    #[inline(always)]
    pub fn int1(&self) -> Int1R {
        Int1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interrupt status for int2.\n\nwhen writte 1, int is cleared."]
    #[inline(always)]
    pub fn int2(&self) -> Int2R {
        Int2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - interrupt status for int3.\n\nwhen writte 1, int is cleared."]
    #[inline(always)]
    pub fn int3(&self) -> Int3R {
        Int3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - interrupt status for int0.\n\nwhen writte 1, int is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn int0(&mut self) -> Int0W<B2aStatusSpec> {
        Int0W::new(self, 0)
    }
    #[doc = "Bit 1 - interrupt status for int1.\n\nwhen writte 1, int is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> Int1W<B2aStatusSpec> {
        Int1W::new(self, 1)
    }
    #[doc = "Bit 2 - interrupt status for int2.\n\nwhen writte 1, int is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn int2(&mut self) -> Int2W<B2aStatusSpec> {
        Int2W::new(self, 2)
    }
    #[doc = "Bit 3 - interrupt status for int3.\n\nwhen writte 1, int is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn int3(&mut self) -> Int3W<B2aStatusSpec> {
        Int3W::new(self, 3)
    }
}
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b2a_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b2a_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B2aStatusSpec;
impl crate::RegisterSpec for B2aStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b2a_status::R`](R) reader structure"]
impl crate::Readable for B2aStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`b2a_status::W`](W) writer structure"]
impl crate::Writable for B2aStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets B2A_STATUS to value 0"]
impl crate::Resettable for B2aStatusSpec {
    const RESET_VALUE: u32 = 0;
}
