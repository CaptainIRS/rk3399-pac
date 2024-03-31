#[doc = "Register `LS_SYNC` reader"]
pub type R = crate::R<LsSyncSpec>;
#[doc = "Register `LS_SYNC` writer"]
pub type W = crate::W<LsSyncSpec>;
#[doc = "Writing a 1 to this register results in all level-sensitive interrupts\n\nbeing synchronized to pclk_intr.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioLsSync {
    #[doc = "0: No synchronization to pclk_intr (default)"]
    B0 = 0,
    #[doc = "1: Synchronize to pclk_intr"]
    B1 = 1,
}
impl From<GpioLsSync> for bool {
    #[inline(always)]
    fn from(variant: GpioLsSync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_LS_SYNC` reader - Writing a 1 to this register results in all level-sensitive interrupts\n\nbeing synchronized to pclk_intr."]
pub type GpioLsSyncR = crate::BitReader<GpioLsSync>;
impl GpioLsSyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioLsSync {
        match self.bits {
            false => GpioLsSync::B0,
            true => GpioLsSync::B1,
        }
    }
    #[doc = "No synchronization to pclk_intr (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GpioLsSync::B0
    }
    #[doc = "Synchronize to pclk_intr"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GpioLsSync::B1
    }
}
#[doc = "Field `GPIO_LS_SYNC` writer - Writing a 1 to this register results in all level-sensitive interrupts\n\nbeing synchronized to pclk_intr."]
pub type GpioLsSyncW<'a, REG> = crate::BitWriter<'a, REG, GpioLsSync>;
impl<'a, REG> GpioLsSyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No synchronization to pclk_intr (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GpioLsSync::B0)
    }
    #[doc = "Synchronize to pclk_intr"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GpioLsSync::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Writing a 1 to this register results in all level-sensitive interrupts\n\nbeing synchronized to pclk_intr."]
    #[inline(always)]
    pub fn gpio_ls_sync(&self) -> GpioLsSyncR {
        GpioLsSyncR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing a 1 to this register results in all level-sensitive interrupts\n\nbeing synchronized to pclk_intr."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_ls_sync(&mut self) -> GpioLsSyncW<LsSyncSpec> {
        GpioLsSyncW::new(self, 0)
    }
}
#[doc = "Level_sensitive synchronization enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ls_sync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ls_sync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LsSyncSpec;
impl crate::RegisterSpec for LsSyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ls_sync::R`](R) reader structure"]
impl crate::Readable for LsSyncSpec {}
#[doc = "`write(|w| ..)` method takes [`ls_sync::W`](W) writer structure"]
impl crate::Writable for LsSyncSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LS_SYNC to value 0"]
impl crate::Resettable for LsSyncSpec {
    const RESET_VALUE: u32 = 0;
}
