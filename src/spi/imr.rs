#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<ImrSpec>;
#[doc = "Transmit FIFO Empty Interrupt Mask\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfeim {
    #[doc = "0: spi_txe_intr interrupt is masked"]
    B0 = 0,
    #[doc = "1: spi_txe_intr interrupt is not masked"]
    B1 = 1,
}
impl From<Tfeim> for bool {
    #[inline(always)]
    fn from(variant: Tfeim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFEIM` reader - Transmit FIFO Empty Interrupt Mask"]
pub type TfeimR = crate::BitReader<Tfeim>;
impl TfeimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfeim {
        match self.bits {
            false => Tfeim::B0,
            true => Tfeim::B1,
        }
    }
    #[doc = "spi_txe_intr interrupt is masked"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tfeim::B0
    }
    #[doc = "spi_txe_intr interrupt is not masked"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tfeim::B1
    }
}
#[doc = "Field `TFEIM` writer - Transmit FIFO Empty Interrupt Mask"]
pub type TfeimW<'a, REG> = crate::BitWriter<'a, REG, Tfeim>;
impl<'a, REG> TfeimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "spi_txe_intr interrupt is masked"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tfeim::B0)
    }
    #[doc = "spi_txe_intr interrupt is not masked"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tfeim::B1)
    }
}
#[doc = "Transmit FIFO Overflow Interrupt Mask\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfoim {
    #[doc = "0: spi_txo_intr interrupt is masked"]
    B0 = 0,
    #[doc = "1: spi_txo_intr interrupt is not masked"]
    B1 = 1,
}
impl From<Tfoim> for bool {
    #[inline(always)]
    fn from(variant: Tfoim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFOIM` reader - Transmit FIFO Overflow Interrupt Mask"]
pub type TfoimR = crate::BitReader<Tfoim>;
impl TfoimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfoim {
        match self.bits {
            false => Tfoim::B0,
            true => Tfoim::B1,
        }
    }
    #[doc = "spi_txo_intr interrupt is masked"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tfoim::B0
    }
    #[doc = "spi_txo_intr interrupt is not masked"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tfoim::B1
    }
}
#[doc = "Field `TFOIM` writer - Transmit FIFO Overflow Interrupt Mask"]
pub type TfoimW<'a, REG> = crate::BitWriter<'a, REG, Tfoim>;
impl<'a, REG> TfoimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "spi_txo_intr interrupt is masked"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tfoim::B0)
    }
    #[doc = "spi_txo_intr interrupt is not masked"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tfoim::B1)
    }
}
#[doc = "Receive FIFO Underflow Interrupt Mask\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfuim {
    #[doc = "0: spi_rxu_intr interrupt is masked"]
    B0 = 0,
    #[doc = "1: spi_rxu_intr interrupt is not masked"]
    B1 = 1,
}
impl From<Rfuim> for bool {
    #[inline(always)]
    fn from(variant: Rfuim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFUIM` reader - Receive FIFO Underflow Interrupt Mask"]
pub type RfuimR = crate::BitReader<Rfuim>;
impl RfuimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfuim {
        match self.bits {
            false => Rfuim::B0,
            true => Rfuim::B1,
        }
    }
    #[doc = "spi_rxu_intr interrupt is masked"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rfuim::B0
    }
    #[doc = "spi_rxu_intr interrupt is not masked"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rfuim::B1
    }
}
#[doc = "Field `RFUIM` writer - Receive FIFO Underflow Interrupt Mask"]
pub type RfuimW<'a, REG> = crate::BitWriter<'a, REG, Rfuim>;
impl<'a, REG> RfuimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "spi_rxu_intr interrupt is masked"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfuim::B0)
    }
    #[doc = "spi_rxu_intr interrupt is not masked"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfuim::B1)
    }
}
#[doc = "Receive FIFO Overflow Interrupt Mask\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfoim {
    #[doc = "0: spi_rxo_intr interrupt is masked"]
    B0 = 0,
    #[doc = "1: spi_rxo_intr interrupt is not masked"]
    B1 = 1,
}
impl From<Rfoim> for bool {
    #[inline(always)]
    fn from(variant: Rfoim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFOIM` reader - Receive FIFO Overflow Interrupt Mask"]
pub type RfoimR = crate::BitReader<Rfoim>;
impl RfoimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfoim {
        match self.bits {
            false => Rfoim::B0,
            true => Rfoim::B1,
        }
    }
    #[doc = "spi_rxo_intr interrupt is masked"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rfoim::B0
    }
    #[doc = "spi_rxo_intr interrupt is not masked"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rfoim::B1
    }
}
#[doc = "Field `RFOIM` writer - Receive FIFO Overflow Interrupt Mask"]
pub type RfoimW<'a, REG> = crate::BitWriter<'a, REG, Rfoim>;
impl<'a, REG> RfoimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "spi_rxo_intr interrupt is masked"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfoim::B0)
    }
    #[doc = "spi_rxo_intr interrupt is not masked"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfoim::B1)
    }
}
#[doc = "Receive FIFO Full Interrupt Mask\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rffim {
    #[doc = "0: spi_rxf_intr interrupt is masked"]
    B0 = 0,
    #[doc = "1: spi_rxf_intr interrupt is not masked"]
    B1 = 1,
}
impl From<Rffim> for bool {
    #[inline(always)]
    fn from(variant: Rffim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFFIM` reader - Receive FIFO Full Interrupt Mask"]
pub type RffimR = crate::BitReader<Rffim>;
impl RffimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rffim {
        match self.bits {
            false => Rffim::B0,
            true => Rffim::B1,
        }
    }
    #[doc = "spi_rxf_intr interrupt is masked"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rffim::B0
    }
    #[doc = "spi_rxf_intr interrupt is not masked"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rffim::B1
    }
}
#[doc = "Field `RFFIM` writer - Receive FIFO Full Interrupt Mask"]
pub type RffimW<'a, REG> = crate::BitWriter<'a, REG, Rffim>;
impl<'a, REG> RffimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "spi_rxf_intr interrupt is masked"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rffim::B0)
    }
    #[doc = "spi_rxf_intr interrupt is not masked"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rffim::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn tfeim(&self) -> TfeimR {
        TfeimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Overflow Interrupt Mask"]
    #[inline(always)]
    pub fn tfoim(&self) -> TfoimR {
        TfoimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO Underflow Interrupt Mask"]
    #[inline(always)]
    pub fn rfuim(&self) -> RfuimR {
        RfuimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Overflow Interrupt Mask"]
    #[inline(always)]
    pub fn rfoim(&self) -> RfoimR {
        RfoimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn rffim(&self) -> RffimR {
        RffimR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn tfeim(&mut self) -> TfeimW<ImrSpec> {
        TfeimW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Overflow Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn tfoim(&mut self) -> TfoimW<ImrSpec> {
        TfoimW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive FIFO Underflow Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rfuim(&mut self) -> RfuimW<ImrSpec> {
        RfuimW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive FIFO Overflow Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rfoim(&mut self) -> RfoimW<ImrSpec> {
        RfoimW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rffim(&mut self) -> RffimW<ImrSpec> {
        RffimW::new(self, 4)
    }
}
#[doc = "Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for ImrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0;
}
