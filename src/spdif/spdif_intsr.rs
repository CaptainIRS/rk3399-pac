#[doc = "Register `SPDIF_INTSR` reader"]
pub type R = crate::R<SpdifIntsrSpec>;
#[doc = "Register `SPDIF_INTSR` writer"]
pub type W = crate::W<SpdifIntsrSpec>;
#[doc = "User Data Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Udtis {
    #[doc = "0: active"]
    B0 = 0,
    #[doc = "1: active"]
    B1 = 1,
}
impl From<Udtis> for bool {
    #[inline(always)]
    fn from(variant: Udtis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDTIS` reader - User Data Interrupt Status"]
pub type UdtisR = crate::BitReader<Udtis>;
impl UdtisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Udtis {
        match self.bits {
            false => Udtis::B0,
            true => Udtis::B1,
        }
    }
    #[doc = "active"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Udtis::B0
    }
    #[doc = "active"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Udtis::B1
    }
}
#[doc = "Field `UDTIS` writer - User Data Interrupt Status"]
pub type UdtisW<'a, REG> = crate::BitWriter<'a, REG, Udtis>;
impl<'a, REG> UdtisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "active"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Udtis::B0)
    }
    #[doc = "active"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Udtis::B1)
    }
}
#[doc = "Block/Data burst transfer interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bttis {
    #[doc = "0: active"]
    B0 = 0,
    #[doc = "1: active"]
    B1 = 1,
}
impl From<Bttis> for bool {
    #[inline(always)]
    fn from(variant: Bttis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BTTIS` reader - Block/Data burst transfer interrupt status"]
pub type BttisR = crate::BitReader<Bttis>;
impl BttisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bttis {
        match self.bits {
            false => Bttis::B0,
            true => Bttis::B1,
        }
    }
    #[doc = "active"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bttis::B0
    }
    #[doc = "active"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bttis::B1
    }
}
#[doc = "Field `BTTIS` writer - Block/Data burst transfer interrupt status"]
pub type BttisW<'a, REG> = crate::BitWriter<'a, REG, Bttis>;
impl<'a, REG> BttisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "active"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bttis::B0)
    }
    #[doc = "active"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bttis::B1)
    }
}
#[doc = "Sample Date Buffer empty interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdbeis {
    #[doc = "0: active"]
    B0 = 0,
    #[doc = "1: active"]
    B1 = 1,
}
impl From<Sdbeis> for bool {
    #[inline(always)]
    fn from(variant: Sdbeis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDBEIS` reader - Sample Date Buffer empty interrupt status"]
pub type SdbeisR = crate::BitReader<Sdbeis>;
impl SdbeisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdbeis {
        match self.bits {
            false => Sdbeis::B0,
            true => Sdbeis::B1,
        }
    }
    #[doc = "active"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sdbeis::B0
    }
    #[doc = "active"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sdbeis::B1
    }
}
#[doc = "Field `SDBEIS` writer - Sample Date Buffer empty interrupt status"]
pub type SdbeisW<'a, REG> = crate::BitWriter<'a, REG, Sdbeis>;
impl<'a, REG> SdbeisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "active"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdbeis::B0)
    }
    #[doc = "active"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdbeis::B1)
    }
}
impl R {
    #[doc = "Bit 2 - User Data Interrupt Status"]
    #[inline(always)]
    pub fn udtis(&self) -> UdtisR {
        UdtisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Block/Data burst transfer interrupt status"]
    #[inline(always)]
    pub fn bttis(&self) -> BttisR {
        BttisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sample Date Buffer empty interrupt status"]
    #[inline(always)]
    pub fn sdbeis(&self) -> SdbeisR {
        SdbeisR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - User Data Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn udtis(&mut self) -> UdtisW<SpdifIntsrSpec> {
        UdtisW::new(self, 2)
    }
    #[doc = "Bit 3 - Block/Data burst transfer interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn bttis(&mut self) -> BttisW<SpdifIntsrSpec> {
        BttisW::new(self, 3)
    }
    #[doc = "Bit 4 - Sample Date Buffer empty interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn sdbeis(&mut self) -> SdbeisW<SpdifIntsrSpec> {
        SdbeisW::new(self, 4)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_intsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdif_intsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdifIntsrSpec;
impl crate::RegisterSpec for SpdifIntsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdif_intsr::R`](R) reader structure"]
impl crate::Readable for SpdifIntsrSpec {}
#[doc = "`write(|w| ..)` method takes [`spdif_intsr::W`](W) writer structure"]
impl crate::Writable for SpdifIntsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPDIF_INTSR to value 0"]
impl crate::Resettable for SpdifIntsrSpec {
    const RESET_VALUE: u32 = 0;
}
