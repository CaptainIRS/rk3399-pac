#[doc = "Register `GMAC_INTF_MODE_STA` reader"]
pub type R = crate::R<GmacIntfModeStaSpec>;
#[doc = "Register `GMAC_INTF_MODE_STA` writer"]
pub type W = crate::W<GmacIntfModeStaSpec>;
#[doc = "Link Mode Indicates the current mode of operation of the link:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lm {
    #[doc = "0: Full-Duplex mode"]
    B0 = 0,
    #[doc = "1: Full-Duplex mode"]
    B1 = 1,
}
impl From<Lm> for bool {
    #[inline(always)]
    fn from(variant: Lm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LM` reader - Link Mode Indicates the current mode of operation of the link:"]
pub type LmR = crate::BitReader<Lm>;
impl LmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lm {
        match self.bits {
            false => Lm::B0,
            true => Lm::B1,
        }
    }
    #[doc = "Full-Duplex mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lm::B0
    }
    #[doc = "Full-Duplex mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lm::B1
    }
}
#[doc = "Field `LM` writer - Link Mode Indicates the current mode of operation of the link:"]
pub type LmW<'a, REG> = crate::BitWriter<'a, REG, Lm>;
impl<'a, REG> LmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Full-Duplex mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lm::B0)
    }
    #[doc = "Full-Duplex mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lm::B1)
    }
}
#[doc = "Link Speed Indicates the current speed of the link:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lsd {
    #[doc = "0: 125 MHz"]
    B00 = 0,
    #[doc = "1: 125 MHz"]
    B01 = 1,
    #[doc = "2: 125 MHz"]
    B10 = 2,
}
impl From<Lsd> for u8 {
    #[inline(always)]
    fn from(variant: Lsd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lsd {
    type Ux = u8;
}
#[doc = "Field `LSD` reader - Link Speed Indicates the current speed of the link:"]
pub type LsdR = crate::FieldReader<Lsd>;
impl LsdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lsd> {
        match self.bits {
            0 => Some(Lsd::B00),
            1 => Some(Lsd::B01),
            2 => Some(Lsd::B10),
            _ => None,
        }
    }
    #[doc = "125 MHz"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Lsd::B00
    }
    #[doc = "125 MHz"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Lsd::B01
    }
    #[doc = "125 MHz"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Lsd::B10
    }
}
#[doc = "Field `LST` reader - Link Status Indicates whether the link is up (1'b1) or down (1'b0)"]
pub type LstR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Link Mode Indicates the current mode of operation of the link:"]
    #[inline(always)]
    pub fn lm(&self) -> LmR {
        LmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Link Speed Indicates the current speed of the link:"]
    #[inline(always)]
    pub fn lsd(&self) -> LsdR {
        LsdR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Link Status Indicates whether the link is up (1'b1) or down (1'b0)"]
    #[inline(always)]
    pub fn lst(&self) -> LstR {
        LstR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Link Mode Indicates the current mode of operation of the link:"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LmW<GmacIntfModeStaSpec> {
        LmW::new(self, 0)
    }
}
#[doc = "RGMII Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_intf_mode_sta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_intf_mode_sta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacIntfModeStaSpec;
impl crate::RegisterSpec for GmacIntfModeStaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_intf_mode_sta::R`](R) reader structure"]
impl crate::Readable for GmacIntfModeStaSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_intf_mode_sta::W`](W) writer structure"]
impl crate::Writable for GmacIntfModeStaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_INTF_MODE_STA to value 0"]
impl crate::Resettable for GmacIntfModeStaSpec {
    const RESET_VALUE: u32 = 0;
}
