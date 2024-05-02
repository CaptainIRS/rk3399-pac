#[doc = "Register `MIPI_CTRL` reader"]
pub type R = crate::R<MipiCtrlSpec>;
#[doc = "Register `MIPI_CTRL` writer"]
pub type W = crate::W<MipiCtrlSpec>;
#[doc = "Field `OUTPUT_ENA` reader - 1: output to add data fifo and to output interface is\n\nenabled 0: output is disabled"]
pub type OutputEnaR = crate::BitReader;
#[doc = "Field `OUTPUT_ENA` writer - 1: output to add data fifo and to output interface is\n\nenabled 0: output is disabled"]
pub type OutputEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLUSH_FIFO` reader - writing '1' resets the write- and read pointers of the\n\nadditional data fifo, reading returns the status of the flush_fifo bit. This bit must be reset by software."]
pub type FlushFifoR = crate::BitReader;
#[doc = "Field `FLUSH_FIFO` writer - writing '1' resets the write- and read pointers of the\n\nadditional data fifo, reading returns the status of the flush_fifo bit. This bit must be reset by software."]
pub type FlushFifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHUTDOWN_LANE` reader - Shutdown Lane Module. Content of this register is\n\ndirectly connected to the output signal shutdown\\[n-1:0\\]\n\nwhere n denotes the lane number 1..4"]
pub type ShutdownLaneR = crate::FieldReader;
#[doc = "Field `SHUTDOWN_LANE` writer - Shutdown Lane Module. Content of this register is\n\ndirectly connected to the output signal shutdown\\[n-1:0\\]\n\nwhere n denotes the lane number 1..4"]
pub type ShutdownLaneW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NumLanes {
    #[doc = "0: Lane 1 is used"]
    B00 = 0,
    #[doc = "1: Lanes 1 and 2 are used"]
    B01 = 1,
    #[doc = "2: Lanes 1, 2 and 3 are used"]
    B10 = 2,
    #[doc = "3: Lanes 1, 2, 3 and 4 are used (default)"]
    B11 = 3,
}
impl From<NumLanes> for u8 {
    #[inline(always)]
    fn from(variant: NumLanes) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NumLanes {
    type Ux = u8;
}
#[doc = "Field `NUM_LANES` reader - "]
pub type NumLanesR = crate::FieldReader<NumLanes>;
impl NumLanesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NumLanes {
        match self.bits {
            0 => NumLanes::B00,
            1 => NumLanes::B01,
            2 => NumLanes::B10,
            3 => NumLanes::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Lane 1 is used"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == NumLanes::B00
    }
    #[doc = "Lanes 1 and 2 are used"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == NumLanes::B01
    }
    #[doc = "Lanes 1, 2 and 3 are used"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == NumLanes::B10
    }
    #[doc = "Lanes 1, 2, 3 and 4 are used (default)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == NumLanes::B11
    }
}
#[doc = "Field `NUM_LANES` writer - "]
pub type NumLanesW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, NumLanes>;
impl<'a, REG> NumLanesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Lane 1 is used"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(NumLanes::B00)
    }
    #[doc = "Lanes 1 and 2 are used"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(NumLanes::B01)
    }
    #[doc = "Lanes 1, 2 and 3 are used"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(NumLanes::B10)
    }
    #[doc = "Lanes 1, 2, 3 and 4 are used (default)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(NumLanes::B11)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrSotHsSkip {
    #[doc = "1: data within the current transmission is skipped if ErrSotHS is detected"]
    B1 = 1,
    #[doc = "0: ErrSotHS does not affect transmission (default)"]
    B0 = 0,
}
impl From<ErrSotHsSkip> for bool {
    #[inline(always)]
    fn from(variant: ErrSotHsSkip) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR_SOT_HS_SKIP` reader - "]
pub type ErrSotHsSkipR = crate::BitReader<ErrSotHsSkip>;
impl ErrSotHsSkipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ErrSotHsSkip {
        match self.bits {
            true => ErrSotHsSkip::B1,
            false => ErrSotHsSkip::B0,
        }
    }
    #[doc = "data within the current transmission is skipped if ErrSotHS is detected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ErrSotHsSkip::B1
    }
    #[doc = "ErrSotHS does not affect transmission (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ErrSotHsSkip::B0
    }
}
#[doc = "Field `ERR_SOT_HS_SKIP` writer - "]
pub type ErrSotHsSkipW<'a, REG> = crate::BitWriter<'a, REG, ErrSotHsSkip>;
impl<'a, REG> ErrSotHsSkipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "data within the current transmission is skipped if ErrSotHS is detected"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ErrSotHsSkip::B1)
    }
    #[doc = "ErrSotHS does not affect transmission (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ErrSotHsSkip::B0)
    }
}
#[doc = "Field `S_ENABLE_CLK` reader - Sensor clock lane enable signal. This register is\n\ndirectly connected to the output port 's_enableclk'.\n\n'1': enable sensor clock lane (DEFAULT) '0': disable\n\nsensor clock lane"]
pub type SEnableClkR = crate::BitReader;
#[doc = "Field `S_ENABLE_CLK` writer - Sensor clock lane enable signal. This register is\n\ndirectly connected to the output port 's_enableclk'.\n\n'1': enable sensor clock lane (DEFAULT) '0': disable\n\nsensor clock lane"]
pub type SEnableClkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: output to add data fifo and to output interface is\n\nenabled 0: output is disabled"]
    #[inline(always)]
    pub fn output_ena(&self) -> OutputEnaR {
        OutputEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - writing '1' resets the write- and read pointers of the\n\nadditional data fifo, reading returns the status of the flush_fifo bit. This bit must be reset by software."]
    #[inline(always)]
    pub fn flush_fifo(&self) -> FlushFifoR {
        FlushFifoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Shutdown Lane Module. Content of this register is\n\ndirectly connected to the output signal shutdown\\[n-1:0\\]\n\nwhere n denotes the lane number 1..4"]
    #[inline(always)]
    pub fn shutdown_lane(&self) -> ShutdownLaneR {
        ShutdownLaneR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn num_lanes(&self) -> NumLanesR {
        NumLanesR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn err_sot_hs_skip(&self) -> ErrSotHsSkipR {
        ErrSotHsSkipR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Sensor clock lane enable signal. This register is\n\ndirectly connected to the output port 's_enableclk'.\n\n'1': enable sensor clock lane (DEFAULT) '0': disable\n\nsensor clock lane"]
    #[inline(always)]
    pub fn s_enable_clk(&self) -> SEnableClkR {
        SEnableClkR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: output to add data fifo and to output interface is\n\nenabled 0: output is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn output_ena(&mut self) -> OutputEnaW<MipiCtrlSpec> {
        OutputEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - writing '1' resets the write- and read pointers of the\n\nadditional data fifo, reading returns the status of the flush_fifo bit. This bit must be reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn flush_fifo(&mut self) -> FlushFifoW<MipiCtrlSpec> {
        FlushFifoW::new(self, 1)
    }
    #[doc = "Bits 8:11 - Shutdown Lane Module. Content of this register is\n\ndirectly connected to the output signal shutdown\\[n-1:0\\]\n\nwhere n denotes the lane number 1..4"]
    #[inline(always)]
    #[must_use]
    pub fn shutdown_lane(&mut self) -> ShutdownLaneW<MipiCtrlSpec> {
        ShutdownLaneW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn num_lanes(&mut self) -> NumLanesW<MipiCtrlSpec> {
        NumLanesW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn err_sot_hs_skip(&mut self) -> ErrSotHsSkipW<MipiCtrlSpec> {
        ErrSotHsSkipW::new(self, 16)
    }
    #[doc = "Bit 18 - Sensor clock lane enable signal. This register is\n\ndirectly connected to the output port 's_enableclk'.\n\n'1': enable sensor clock lane (DEFAULT) '0': disable\n\nsensor clock lane"]
    #[inline(always)]
    #[must_use]
    pub fn s_enable_clk(&mut self) -> SEnableClkW<MipiCtrlSpec> {
        SEnableClkW::new(self, 18)
    }
}
#[doc = "global control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mipi_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiCtrlSpec;
impl crate::RegisterSpec for MipiCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_ctrl::R`](R) reader structure"]
impl crate::Readable for MipiCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mipi_ctrl::W`](W) writer structure"]
impl crate::Writable for MipiCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIPI_CTRL to value 0x0006_0000"]
impl crate::Resettable for MipiCtrlSpec {
    const RESET_VALUE: u32 = 0x0006_0000;
}
