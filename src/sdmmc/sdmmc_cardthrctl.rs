#[doc = "Register `SDMMC_CARDTHRCTL` reader"]
pub type R = crate::R<SdmmcCardthrctlSpec>;
#[doc = "Register `SDMMC_CARDTHRCTL` writer"]
pub type W = crate::W<SdmmcCardthrctlSpec>;
#[doc = "Card Read Threshold Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardrdthren {
    #[doc = "0: Card Read Threshold enabled. Host Controller initiates Read Transfer only if CardRdThreshold amount of space is available in receive FIFO."]
    B0 = 0,
    #[doc = "1: Card Read Threshold enabled. Host Controller initiates Read Transfer only if CardRdThreshold amount of space is available in receive FIFO."]
    B1 = 1,
}
impl From<Cardrdthren> for bool {
    #[inline(always)]
    fn from(variant: Cardrdthren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDRDTHREN` reader - Card Read Threshold Enable."]
pub type CardrdthrenR = crate::BitReader<Cardrdthren>;
impl CardrdthrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cardrdthren {
        match self.bits {
            false => Cardrdthren::B0,
            true => Cardrdthren::B1,
        }
    }
    #[doc = "Card Read Threshold enabled. Host Controller initiates Read Transfer only if CardRdThreshold amount of space is available in receive FIFO."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cardrdthren::B0
    }
    #[doc = "Card Read Threshold enabled. Host Controller initiates Read Transfer only if CardRdThreshold amount of space is available in receive FIFO."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cardrdthren::B1
    }
}
#[doc = "Field `CARDRDTHREN` writer - Card Read Threshold Enable."]
pub type CardrdthrenW<'a, REG> = crate::BitWriter<'a, REG, Cardrdthren>;
impl<'a, REG> CardrdthrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Card Read Threshold enabled. Host Controller initiates Read Transfer only if CardRdThreshold amount of space is available in receive FIFO."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cardrdthren::B0)
    }
    #[doc = "Card Read Threshold enabled. Host Controller initiates Read Transfer only if CardRdThreshold amount of space is available in receive FIFO."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cardrdthren::B1)
    }
}
#[doc = "Busy Clear Interrupt generation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bsyclrinten {
    #[doc = "0: Busy Clear Interrupt enabled Note: The application can disable this feature if it does not want to wait for a Busy Clear Interrupt. For example, in a multi-card scenario, the application can switch to the other card without waiting for a busy to be completed. In such cases, the application can use the polling method to determine the status of busy. By default this feature is disabled and backward-compatible to the legacy drivers where polling is used."]
    B0 = 0,
    #[doc = "1: Busy Clear Interrupt enabled Note: The application can disable this feature if it does not want to wait for a Busy Clear Interrupt. For example, in a multi-card scenario, the application can switch to the other card without waiting for a busy to be completed. In such cases, the application can use the polling method to determine the status of busy. By default this feature is disabled and backward-compatible to the legacy drivers where polling is used."]
    B1 = 1,
}
impl From<Bsyclrinten> for bool {
    #[inline(always)]
    fn from(variant: Bsyclrinten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSYCLRINTEN` reader - Busy Clear Interrupt generation:"]
pub type BsyclrintenR = crate::BitReader<Bsyclrinten>;
impl BsyclrintenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bsyclrinten {
        match self.bits {
            false => Bsyclrinten::B0,
            true => Bsyclrinten::B1,
        }
    }
    #[doc = "Busy Clear Interrupt enabled Note: The application can disable this feature if it does not want to wait for a Busy Clear Interrupt. For example, in a multi-card scenario, the application can switch to the other card without waiting for a busy to be completed. In such cases, the application can use the polling method to determine the status of busy. By default this feature is disabled and backward-compatible to the legacy drivers where polling is used."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bsyclrinten::B0
    }
    #[doc = "Busy Clear Interrupt enabled Note: The application can disable this feature if it does not want to wait for a Busy Clear Interrupt. For example, in a multi-card scenario, the application can switch to the other card without waiting for a busy to be completed. In such cases, the application can use the polling method to determine the status of busy. By default this feature is disabled and backward-compatible to the legacy drivers where polling is used."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bsyclrinten::B1
    }
}
#[doc = "Field `BSYCLRINTEN` writer - Busy Clear Interrupt generation:"]
pub type BsyclrintenW<'a, REG> = crate::BitWriter<'a, REG, Bsyclrinten>;
impl<'a, REG> BsyclrintenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Busy Clear Interrupt enabled Note: The application can disable this feature if it does not want to wait for a Busy Clear Interrupt. For example, in a multi-card scenario, the application can switch to the other card without waiting for a busy to be completed. In such cases, the application can use the polling method to determine the status of busy. By default this feature is disabled and backward-compatible to the legacy drivers where polling is used."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bsyclrinten::B0)
    }
    #[doc = "Busy Clear Interrupt enabled Note: The application can disable this feature if it does not want to wait for a Busy Clear Interrupt. For example, in a multi-card scenario, the application can switch to the other card without waiting for a busy to be completed. In such cases, the application can use the polling method to determine the status of busy. By default this feature is disabled and backward-compatible to the legacy drivers where polling is used."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bsyclrinten::B1)
    }
}
#[doc = "Field `CARDRDTHRESHOLD` reader - Card Read Threshold size"]
pub type CardrdthresholdR = crate::FieldReader<u16>;
#[doc = "Field `CARDRDTHRESHOLD` writer - Card Read Threshold size"]
pub type CardrdthresholdW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - Card Read Threshold Enable."]
    #[inline(always)]
    pub fn cardrdthren(&self) -> CardrdthrenR {
        CardrdthrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Busy Clear Interrupt generation:"]
    #[inline(always)]
    pub fn bsyclrinten(&self) -> BsyclrintenR {
        BsyclrintenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Card Read Threshold size"]
    #[inline(always)]
    pub fn cardrdthreshold(&self) -> CardrdthresholdR {
        CardrdthresholdR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Card Read Threshold Enable."]
    #[inline(always)]
    #[must_use]
    pub fn cardrdthren(&mut self) -> CardrdthrenW<SdmmcCardthrctlSpec> {
        CardrdthrenW::new(self, 0)
    }
    #[doc = "Bit 1 - Busy Clear Interrupt generation:"]
    #[inline(always)]
    #[must_use]
    pub fn bsyclrinten(&mut self) -> BsyclrintenW<SdmmcCardthrctlSpec> {
        BsyclrintenW::new(self, 1)
    }
    #[doc = "Bits 16:27 - Card Read Threshold size"]
    #[inline(always)]
    #[must_use]
    pub fn cardrdthreshold(&mut self) -> CardrdthresholdW<SdmmcCardthrctlSpec> {
        CardrdthresholdW::new(self, 16)
    }
}
#[doc = "Card read threshold enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_cardthrctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_cardthrctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcCardthrctlSpec;
impl crate::RegisterSpec for SdmmcCardthrctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_cardthrctl::R`](R) reader structure"]
impl crate::Readable for SdmmcCardthrctlSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_cardthrctl::W`](W) writer structure"]
impl crate::Writable for SdmmcCardthrctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_CARDTHRCTL to value 0"]
impl crate::Resettable for SdmmcCardthrctlSpec {
    const RESET_VALUE: u32 = 0;
}
