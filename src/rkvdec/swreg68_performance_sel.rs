#[doc = "Register `SWREG68_PERFORMANCE_SEL` reader"]
pub type R = crate::R<Swreg68PerformanceSelSpec>;
#[doc = "Register `SWREG68_PERFORMANCE_SEL` writer"]
pub type W = crate::W<Swreg68PerformanceSelSpec>;
#[doc = "sel counter0 to cal which signal\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PerfCnt0Sel {
    #[doc = "0: don't work;"]
    D0 = 0,
    #[doc = "1: cycles counter for cabac in buffer empty;"]
    D1 = 1,
    #[doc = "2: cycles counter for cabac in buffer full;"]
    D2 = 2,
    #[doc = "3: cycles counter for cabac out buffer empty;"]
    D3 = 3,
    #[doc = "4: cycles counter for cabac out buffer full;"]
    D4 = 4,
    #[doc = "5: cycles counter for transd input data ready;"]
    D5 = 5,
    #[doc = "6: cycles counter for transd write data to recon allow;"]
    D6 = 6,
    #[doc = "7: cycles counter for dec2transd cmd empty;"]
    D7 = 7,
    #[doc = "8: cycles counter for dec2transd cmd full;"]
    D8 = 8,
    #[doc = "9: cycles counter for transd2dblk bs fifo empty;"]
    D9 = 9,
    #[doc = "10: cycles counter for transd2dblk bs fifo full;"]
    D10 = 10,
    #[doc = "11: cycles counter for dec2intra cmd fifo empty;"]
    D11 = 11,
    #[doc = "12: cycles counter for dec2intra cmd fifo full;"]
    D12 = 12,
    #[doc = "13: cycles counter for mc2recon cmd fifo empty;"]
    D13 = 13,
    #[doc = "14: cycles counter for mc2recon cmd fifo full;"]
    D14 = 14,
    #[doc = "15: cycles counter for mc2recon data fifo empty;"]
    D15 = 15,
    #[doc = "16: cycles counter for mc2recon data fifo full;"]
    D16 = 16,
    #[doc = "17: cycles counter for recon2filter data write allow;"]
    D17 = 17,
    #[doc = "18: cycles counter for inter2busifd cmd fifo empty;"]
    D18 = 18,
    #[doc = "19: cycles counter for inter2busifd cmd fifo full;"]
    D19 = 19,
    #[doc = "20: cycles counter for busifd2mc data fifo empty;"]
    D20 = 20,
    #[doc = "21: cycles counter for busifd2mc data fifo full;"]
    D21 = 21,
    #[doc = "22: cycles counter for bus working status;"]
    D22 = 22,
    #[doc = "23: cycles counter for dec2inter cmd fifo empty;"]
    D23 = 23,
    #[doc = "24: cycles counter for dec2inter cmd fifo full;"]
    D24 = 24,
    #[doc = "25: cycles counter for inter2mc cmd fifo empty;"]
    D25 = 25,
    #[doc = "26: cycles counter for inter2mc cmd fifo full;"]
    D26 = 26,
    #[doc = "27: cycles counter for inter2dblk bs fifo empty;"]
    D27 = 27,
    #[doc = "28: cycles counter for inter2dblk bs fifo full;"]
    D28 = 28,
    #[doc = "29: cycles counter for colmv_rbuf_empty;"]
    D29 = 29,
    #[doc = "30: cycles counter for colmv_rbuf_full;"]
    D30 = 30,
    #[doc = "31: cycles counter for colmv_wbuf_empty;"]
    D31 = 31,
    #[doc = "32: cycles counter for colmv_wbuf_da_full;"]
    D32 = 32,
    #[doc = "33: cycles counter for dblk input data valid;"]
    D33 = 33,
    #[doc = "34: cycles counter for dblk can't write data to sao;"]
    D34 = 34,
    #[doc = "35: cycles counter for dec2loopfilter cmd fifo empty;"]
    D35 = 35,
    #[doc = "36: cycles counter for dec2loopfilter cmd fifo full;"]
    D36 = 36,
    #[doc = "37: cycles counter for sao input data valid;"]
    D37 = 37,
    #[doc = "38: cycles counter for busifd hold back sao write data;"]
    D38 = 38,
    #[doc = "39: cycles counter for sao output data valid;"]
    D39 = 39,
    #[doc = "40: counter for dec_ctrl read cmd num"]
    D40 = 40,
}
impl From<PerfCnt0Sel> for u8 {
    #[inline(always)]
    fn from(variant: PerfCnt0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PerfCnt0Sel {
    type Ux = u8;
}
#[doc = "Field `PERF_CNT0_SEL` reader - sel counter0 to cal which signal"]
pub type PerfCnt0SelR = crate::FieldReader<PerfCnt0Sel>;
impl PerfCnt0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PerfCnt0Sel> {
        match self.bits {
            0 => Some(PerfCnt0Sel::D0),
            1 => Some(PerfCnt0Sel::D1),
            2 => Some(PerfCnt0Sel::D2),
            3 => Some(PerfCnt0Sel::D3),
            4 => Some(PerfCnt0Sel::D4),
            5 => Some(PerfCnt0Sel::D5),
            6 => Some(PerfCnt0Sel::D6),
            7 => Some(PerfCnt0Sel::D7),
            8 => Some(PerfCnt0Sel::D8),
            9 => Some(PerfCnt0Sel::D9),
            10 => Some(PerfCnt0Sel::D10),
            11 => Some(PerfCnt0Sel::D11),
            12 => Some(PerfCnt0Sel::D12),
            13 => Some(PerfCnt0Sel::D13),
            14 => Some(PerfCnt0Sel::D14),
            15 => Some(PerfCnt0Sel::D15),
            16 => Some(PerfCnt0Sel::D16),
            17 => Some(PerfCnt0Sel::D17),
            18 => Some(PerfCnt0Sel::D18),
            19 => Some(PerfCnt0Sel::D19),
            20 => Some(PerfCnt0Sel::D20),
            21 => Some(PerfCnt0Sel::D21),
            22 => Some(PerfCnt0Sel::D22),
            23 => Some(PerfCnt0Sel::D23),
            24 => Some(PerfCnt0Sel::D24),
            25 => Some(PerfCnt0Sel::D25),
            26 => Some(PerfCnt0Sel::D26),
            27 => Some(PerfCnt0Sel::D27),
            28 => Some(PerfCnt0Sel::D28),
            29 => Some(PerfCnt0Sel::D29),
            30 => Some(PerfCnt0Sel::D30),
            31 => Some(PerfCnt0Sel::D31),
            32 => Some(PerfCnt0Sel::D32),
            33 => Some(PerfCnt0Sel::D33),
            34 => Some(PerfCnt0Sel::D34),
            35 => Some(PerfCnt0Sel::D35),
            36 => Some(PerfCnt0Sel::D36),
            37 => Some(PerfCnt0Sel::D37),
            38 => Some(PerfCnt0Sel::D38),
            39 => Some(PerfCnt0Sel::D39),
            40 => Some(PerfCnt0Sel::D40),
            _ => None,
        }
    }
    #[doc = "don't work;"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == PerfCnt0Sel::D0
    }
    #[doc = "cycles counter for cabac in buffer empty;"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == PerfCnt0Sel::D1
    }
    #[doc = "cycles counter for cabac in buffer full;"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == PerfCnt0Sel::D2
    }
    #[doc = "cycles counter for cabac out buffer empty;"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == PerfCnt0Sel::D3
    }
    #[doc = "cycles counter for cabac out buffer full;"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == PerfCnt0Sel::D4
    }
    #[doc = "cycles counter for transd input data ready;"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == PerfCnt0Sel::D5
    }
    #[doc = "cycles counter for transd write data to recon allow;"]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == PerfCnt0Sel::D6
    }
    #[doc = "cycles counter for dec2transd cmd empty;"]
    #[inline(always)]
    pub fn is_d7(&self) -> bool {
        *self == PerfCnt0Sel::D7
    }
    #[doc = "cycles counter for dec2transd cmd full;"]
    #[inline(always)]
    pub fn is_d8(&self) -> bool {
        *self == PerfCnt0Sel::D8
    }
    #[doc = "cycles counter for transd2dblk bs fifo empty;"]
    #[inline(always)]
    pub fn is_d9(&self) -> bool {
        *self == PerfCnt0Sel::D9
    }
    #[doc = "cycles counter for transd2dblk bs fifo full;"]
    #[inline(always)]
    pub fn is_d10(&self) -> bool {
        *self == PerfCnt0Sel::D10
    }
    #[doc = "cycles counter for dec2intra cmd fifo empty;"]
    #[inline(always)]
    pub fn is_d11(&self) -> bool {
        *self == PerfCnt0Sel::D11
    }
    #[doc = "cycles counter for dec2intra cmd fifo full;"]
    #[inline(always)]
    pub fn is_d12(&self) -> bool {
        *self == PerfCnt0Sel::D12
    }
    #[doc = "cycles counter for mc2recon cmd fifo empty;"]
    #[inline(always)]
    pub fn is_d13(&self) -> bool {
        *self == PerfCnt0Sel::D13
    }
    #[doc = "cycles counter for mc2recon cmd fifo full;"]
    #[inline(always)]
    pub fn is_d14(&self) -> bool {
        *self == PerfCnt0Sel::D14
    }
    #[doc = "cycles counter for mc2recon data fifo empty;"]
    #[inline(always)]
    pub fn is_d15(&self) -> bool {
        *self == PerfCnt0Sel::D15
    }
    #[doc = "cycles counter for mc2recon data fifo full;"]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == PerfCnt0Sel::D16
    }
    #[doc = "cycles counter for recon2filter data write allow;"]
    #[inline(always)]
    pub fn is_d17(&self) -> bool {
        *self == PerfCnt0Sel::D17
    }
    #[doc = "cycles counter for inter2busifd cmd fifo empty;"]
    #[inline(always)]
    pub fn is_d18(&self) -> bool {
        *self == PerfCnt0Sel::D18
    }
    #[doc = "cycles counter for inter2busifd cmd fifo full;"]
    #[inline(always)]
    pub fn is_d19(&self) -> bool {
        *self == PerfCnt0Sel::D19
    }
    #[doc = "cycles counter for busifd2mc data fifo empty;"]
    #[inline(always)]
    pub fn is_d20(&self) -> bool {
        *self == PerfCnt0Sel::D20
    }
    #[doc = "cycles counter for busifd2mc data fifo full;"]
    #[inline(always)]
    pub fn is_d21(&self) -> bool {
        *self == PerfCnt0Sel::D21
    }
    #[doc = "cycles counter for bus working status;"]
    #[inline(always)]
    pub fn is_d22(&self) -> bool {
        *self == PerfCnt0Sel::D22
    }
    #[doc = "cycles counter for dec2inter cmd fifo empty;"]
    #[inline(always)]
    pub fn is_d23(&self) -> bool {
        *self == PerfCnt0Sel::D23
    }
    #[doc = "cycles counter for dec2inter cmd fifo full;"]
    #[inline(always)]
    pub fn is_d24(&self) -> bool {
        *self == PerfCnt0Sel::D24
    }
    #[doc = "cycles counter for inter2mc cmd fifo empty;"]
    #[inline(always)]
    pub fn is_d25(&self) -> bool {
        *self == PerfCnt0Sel::D25
    }
    #[doc = "cycles counter for inter2mc cmd fifo full;"]
    #[inline(always)]
    pub fn is_d26(&self) -> bool {
        *self == PerfCnt0Sel::D26
    }
    #[doc = "cycles counter for inter2dblk bs fifo empty;"]
    #[inline(always)]
    pub fn is_d27(&self) -> bool {
        *self == PerfCnt0Sel::D27
    }
    #[doc = "cycles counter for inter2dblk bs fifo full;"]
    #[inline(always)]
    pub fn is_d28(&self) -> bool {
        *self == PerfCnt0Sel::D28
    }
    #[doc = "cycles counter for colmv_rbuf_empty;"]
    #[inline(always)]
    pub fn is_d29(&self) -> bool {
        *self == PerfCnt0Sel::D29
    }
    #[doc = "cycles counter for colmv_rbuf_full;"]
    #[inline(always)]
    pub fn is_d30(&self) -> bool {
        *self == PerfCnt0Sel::D30
    }
    #[doc = "cycles counter for colmv_wbuf_empty;"]
    #[inline(always)]
    pub fn is_d31(&self) -> bool {
        *self == PerfCnt0Sel::D31
    }
    #[doc = "cycles counter for colmv_wbuf_da_full;"]
    #[inline(always)]
    pub fn is_d32(&self) -> bool {
        *self == PerfCnt0Sel::D32
    }
    #[doc = "cycles counter for dblk input data valid;"]
    #[inline(always)]
    pub fn is_d33(&self) -> bool {
        *self == PerfCnt0Sel::D33
    }
    #[doc = "cycles counter for dblk can't write data to sao;"]
    #[inline(always)]
    pub fn is_d34(&self) -> bool {
        *self == PerfCnt0Sel::D34
    }
    #[doc = "cycles counter for dec2loopfilter cmd fifo empty;"]
    #[inline(always)]
    pub fn is_d35(&self) -> bool {
        *self == PerfCnt0Sel::D35
    }
    #[doc = "cycles counter for dec2loopfilter cmd fifo full;"]
    #[inline(always)]
    pub fn is_d36(&self) -> bool {
        *self == PerfCnt0Sel::D36
    }
    #[doc = "cycles counter for sao input data valid;"]
    #[inline(always)]
    pub fn is_d37(&self) -> bool {
        *self == PerfCnt0Sel::D37
    }
    #[doc = "cycles counter for busifd hold back sao write data;"]
    #[inline(always)]
    pub fn is_d38(&self) -> bool {
        *self == PerfCnt0Sel::D38
    }
    #[doc = "cycles counter for sao output data valid;"]
    #[inline(always)]
    pub fn is_d39(&self) -> bool {
        *self == PerfCnt0Sel::D39
    }
    #[doc = "counter for dec_ctrl read cmd num"]
    #[inline(always)]
    pub fn is_d40(&self) -> bool {
        *self == PerfCnt0Sel::D40
    }
}
#[doc = "Field `PERF_CNT0_SEL` writer - sel counter0 to cal which signal"]
pub type PerfCnt0SelW<'a, REG> = crate::FieldWriter<'a, REG, 6, PerfCnt0Sel>;
impl<'a, REG> PerfCnt0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "don't work;"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D0)
    }
    #[doc = "cycles counter for cabac in buffer empty;"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D1)
    }
    #[doc = "cycles counter for cabac in buffer full;"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D2)
    }
    #[doc = "cycles counter for cabac out buffer empty;"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D3)
    }
    #[doc = "cycles counter for cabac out buffer full;"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D4)
    }
    #[doc = "cycles counter for transd input data ready;"]
    #[inline(always)]
    pub fn d5(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D5)
    }
    #[doc = "cycles counter for transd write data to recon allow;"]
    #[inline(always)]
    pub fn d6(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D6)
    }
    #[doc = "cycles counter for dec2transd cmd empty;"]
    #[inline(always)]
    pub fn d7(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D7)
    }
    #[doc = "cycles counter for dec2transd cmd full;"]
    #[inline(always)]
    pub fn d8(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D8)
    }
    #[doc = "cycles counter for transd2dblk bs fifo empty;"]
    #[inline(always)]
    pub fn d9(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D9)
    }
    #[doc = "cycles counter for transd2dblk bs fifo full;"]
    #[inline(always)]
    pub fn d10(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D10)
    }
    #[doc = "cycles counter for dec2intra cmd fifo empty;"]
    #[inline(always)]
    pub fn d11(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D11)
    }
    #[doc = "cycles counter for dec2intra cmd fifo full;"]
    #[inline(always)]
    pub fn d12(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D12)
    }
    #[doc = "cycles counter for mc2recon cmd fifo empty;"]
    #[inline(always)]
    pub fn d13(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D13)
    }
    #[doc = "cycles counter for mc2recon cmd fifo full;"]
    #[inline(always)]
    pub fn d14(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D14)
    }
    #[doc = "cycles counter for mc2recon data fifo empty;"]
    #[inline(always)]
    pub fn d15(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D15)
    }
    #[doc = "cycles counter for mc2recon data fifo full;"]
    #[inline(always)]
    pub fn d16(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D16)
    }
    #[doc = "cycles counter for recon2filter data write allow;"]
    #[inline(always)]
    pub fn d17(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D17)
    }
    #[doc = "cycles counter for inter2busifd cmd fifo empty;"]
    #[inline(always)]
    pub fn d18(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D18)
    }
    #[doc = "cycles counter for inter2busifd cmd fifo full;"]
    #[inline(always)]
    pub fn d19(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D19)
    }
    #[doc = "cycles counter for busifd2mc data fifo empty;"]
    #[inline(always)]
    pub fn d20(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D20)
    }
    #[doc = "cycles counter for busifd2mc data fifo full;"]
    #[inline(always)]
    pub fn d21(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D21)
    }
    #[doc = "cycles counter for bus working status;"]
    #[inline(always)]
    pub fn d22(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D22)
    }
    #[doc = "cycles counter for dec2inter cmd fifo empty;"]
    #[inline(always)]
    pub fn d23(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D23)
    }
    #[doc = "cycles counter for dec2inter cmd fifo full;"]
    #[inline(always)]
    pub fn d24(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D24)
    }
    #[doc = "cycles counter for inter2mc cmd fifo empty;"]
    #[inline(always)]
    pub fn d25(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D25)
    }
    #[doc = "cycles counter for inter2mc cmd fifo full;"]
    #[inline(always)]
    pub fn d26(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D26)
    }
    #[doc = "cycles counter for inter2dblk bs fifo empty;"]
    #[inline(always)]
    pub fn d27(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D27)
    }
    #[doc = "cycles counter for inter2dblk bs fifo full;"]
    #[inline(always)]
    pub fn d28(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D28)
    }
    #[doc = "cycles counter for colmv_rbuf_empty;"]
    #[inline(always)]
    pub fn d29(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D29)
    }
    #[doc = "cycles counter for colmv_rbuf_full;"]
    #[inline(always)]
    pub fn d30(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D30)
    }
    #[doc = "cycles counter for colmv_wbuf_empty;"]
    #[inline(always)]
    pub fn d31(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D31)
    }
    #[doc = "cycles counter for colmv_wbuf_da_full;"]
    #[inline(always)]
    pub fn d32(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D32)
    }
    #[doc = "cycles counter for dblk input data valid;"]
    #[inline(always)]
    pub fn d33(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D33)
    }
    #[doc = "cycles counter for dblk can't write data to sao;"]
    #[inline(always)]
    pub fn d34(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D34)
    }
    #[doc = "cycles counter for dec2loopfilter cmd fifo empty;"]
    #[inline(always)]
    pub fn d35(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D35)
    }
    #[doc = "cycles counter for dec2loopfilter cmd fifo full;"]
    #[inline(always)]
    pub fn d36(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D36)
    }
    #[doc = "cycles counter for sao input data valid;"]
    #[inline(always)]
    pub fn d37(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D37)
    }
    #[doc = "cycles counter for busifd hold back sao write data;"]
    #[inline(always)]
    pub fn d38(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D38)
    }
    #[doc = "cycles counter for sao output data valid;"]
    #[inline(always)]
    pub fn d39(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D39)
    }
    #[doc = "counter for dec_ctrl read cmd num"]
    #[inline(always)]
    pub fn d40(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt0Sel::D40)
    }
}
#[doc = "Field0000 Abstract\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PerfCnt1Sel {
    #[doc = "0: don't work;"]
    D0 = 0,
    #[doc = "1: cycles counter for cabac in buffer empty;"]
    D1 = 1,
    #[doc = "2: cycles counter for cabac in buffer full;"]
    D2 = 2,
    #[doc = "3: cycles counter for cabac out buffer empty;"]
    D3 = 3,
    #[doc = "4: cycles counter for cabac out buffer full;"]
    D4 = 4,
    #[doc = "5: cycles counter for transd input data ready;"]
    D5 = 5,
    #[doc = "6: cycles counter for transd write data to recon allow;"]
    D6 = 6,
    #[doc = "7: cycles counter for dec2transd cmd empty;"]
    D7 = 7,
    #[doc = "8: cycles counter for dec2transd cmd full;"]
    D8 = 8,
    #[doc = "9: cycles counter for transd2dblk bs fifo empty;"]
    D9 = 9,
    #[doc = "10: cycles counter for transd2dblk bs fifo full;"]
    D10 = 10,
    #[doc = "11: cycles counter for dec2intra cmd fifo empty;"]
    D11 = 11,
    #[doc = "12: cycles counter for dec2intra cmd fifo full;"]
    D12 = 12,
    #[doc = "13: cycles counter for mc2recon cmd fifo empty;"]
    D13 = 13,
    #[doc = "14: cycles counter for mc2recon cmd fifo full;"]
    D14 = 14,
    #[doc = "15: cycles counter for mc2recon data fifo empty;"]
    D15 = 15,
    #[doc = "16: cycles counter for mc2recon data fifo full;"]
    D16 = 16,
    #[doc = "17: cycles counter for recon2filter data write allow;"]
    D17 = 17,
    #[doc = "18: cycles counter for inter2busifd cmd fifo empty;"]
    D18 = 18,
    #[doc = "19: cycles counter for inter2busifd cmd fifo full;"]
    D19 = 19,
    #[doc = "20: cycles counter for busifd2mc data fifo empty;"]
    D20 = 20,
    #[doc = "21: cycles counter for busifd2mc data fifo full;"]
    D21 = 21,
    #[doc = "22: cycles counter for bus working status;"]
    D22 = 22,
    #[doc = "23: cycles counter for dec2inter cmd fifo empty;"]
    D23 = 23,
    #[doc = "24: cycles counter for dec2inter cmd fifo full;"]
    D24 = 24,
    #[doc = "25: cycles counter for inter2mc cmd fifo empty;"]
    D25 = 25,
    #[doc = "26: cycles counter for inter2mc cmd fifo full;"]
    D26 = 26,
    #[doc = "27: cycles counter for inter2dblk bs fifo empty;"]
    D27 = 27,
    #[doc = "28: cycles counter for inter2dblk bs fifo full;"]
    D28 = 28,
    #[doc = "29: cycles counter for colmv_rbuf_empty;"]
    D29 = 29,
    #[doc = "30: cycles counter for colmv_rbuf_full;"]
    D30 = 30,
    #[doc = "31: cycles counter for colmv_wbuf_empty;"]
    D31 = 31,
    #[doc = "32: cycles counter for colmv_wbuf_da_full;"]
    D32 = 32,
    #[doc = "33: cycles counter for dblk input data valid;"]
    D33 = 33,
    #[doc = "34: cycles counter for dblk can't write data to sao;"]
    D34 = 34,
    #[doc = "35: cycles counter for dec2loopfilter cmd fifo empty;"]
    D35 = 35,
    #[doc = "36: cycles counter for dec2loopfilter cmd fifo full;"]
    D36 = 36,
    #[doc = "37: cycles counter for sao input data valid;"]
    D37 = 37,
    #[doc = "38: cycles counter for busifd hold back sao write data;"]
    D38 = 38,
    #[doc = "39: cycles counter for sao output data valid;"]
    D39 = 39,
    #[doc = "40: counter for dec_ctrl read cmd num"]
    D40 = 40,
}
impl From<PerfCnt1Sel> for u8 {
    #[inline(always)]
    fn from(variant: PerfCnt1Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PerfCnt1Sel {
    type Ux = u8;
}
#[doc = "Field `PERF_CNT1_SEL` reader - Field0000 Abstract"]
pub type PerfCnt1SelR = crate::FieldReader<PerfCnt1Sel>;
impl PerfCnt1SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PerfCnt1Sel> {
        match self.bits {
            0 => Some(PerfCnt1Sel::D0),
            1 => Some(PerfCnt1Sel::D1),
            2 => Some(PerfCnt1Sel::D2),
            3 => Some(PerfCnt1Sel::D3),
            4 => Some(PerfCnt1Sel::D4),
            5 => Some(PerfCnt1Sel::D5),
            6 => Some(PerfCnt1Sel::D6),
            7 => Some(PerfCnt1Sel::D7),
            8 => Some(PerfCnt1Sel::D8),
            9 => Some(PerfCnt1Sel::D9),
            10 => Some(PerfCnt1Sel::D10),
            11 => Some(PerfCnt1Sel::D11),
            12 => Some(PerfCnt1Sel::D12),
            13 => Some(PerfCnt1Sel::D13),
            14 => Some(PerfCnt1Sel::D14),
            15 => Some(PerfCnt1Sel::D15),
            16 => Some(PerfCnt1Sel::D16),
            17 => Some(PerfCnt1Sel::D17),
            18 => Some(PerfCnt1Sel::D18),
            19 => Some(PerfCnt1Sel::D19),
            20 => Some(PerfCnt1Sel::D20),
            21 => Some(PerfCnt1Sel::D21),
            22 => Some(PerfCnt1Sel::D22),
            23 => Some(PerfCnt1Sel::D23),
            24 => Some(PerfCnt1Sel::D24),
            25 => Some(PerfCnt1Sel::D25),
            26 => Some(PerfCnt1Sel::D26),
            27 => Some(PerfCnt1Sel::D27),
            28 => Some(PerfCnt1Sel::D28),
            29 => Some(PerfCnt1Sel::D29),
            30 => Some(PerfCnt1Sel::D30),
            31 => Some(PerfCnt1Sel::D31),
            32 => Some(PerfCnt1Sel::D32),
            33 => Some(PerfCnt1Sel::D33),
            34 => Some(PerfCnt1Sel::D34),
            35 => Some(PerfCnt1Sel::D35),
            36 => Some(PerfCnt1Sel::D36),
            37 => Some(PerfCnt1Sel::D37),
            38 => Some(PerfCnt1Sel::D38),
            39 => Some(PerfCnt1Sel::D39),
            40 => Some(PerfCnt1Sel::D40),
            _ => None,
        }
    }
    #[doc = "don't work;"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == PerfCnt1Sel::D0
    }
    #[doc = "cycles counter for cabac in buffer empty;"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == PerfCnt1Sel::D1
    }
    #[doc = "cycles counter for cabac in buffer full;"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == PerfCnt1Sel::D2
    }
    #[doc = "cycles counter for cabac out buffer empty;"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == PerfCnt1Sel::D3
    }
    #[doc = "cycles counter for cabac out buffer full;"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == PerfCnt1Sel::D4
    }
    #[doc = "cycles counter for transd input data ready;"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == PerfCnt1Sel::D5
    }
    #[doc = "cycles counter for transd write data to recon allow;"]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == PerfCnt1Sel::D6
    }
    #[doc = "cycles counter for dec2transd cmd empty;"]
    #[inline(always)]
    pub fn is_d7(&self) -> bool {
        *self == PerfCnt1Sel::D7
    }
    #[doc = "cycles counter for dec2transd cmd full;"]
    #[inline(always)]
    pub fn is_d8(&self) -> bool {
        *self == PerfCnt1Sel::D8
    }
    #[doc = "cycles counter for transd2dblk bs fifo empty;"]
    #[inline(always)]
    pub fn is_d9(&self) -> bool {
        *self == PerfCnt1Sel::D9
    }
    #[doc = "cycles counter for transd2dblk bs fifo full;"]
    #[inline(always)]
    pub fn is_d10(&self) -> bool {
        *self == PerfCnt1Sel::D10
    }
    #[doc = "cycles counter for dec2intra cmd fifo empty;"]
    #[inline(always)]
    pub fn is_d11(&self) -> bool {
        *self == PerfCnt1Sel::D11
    }
    #[doc = "cycles counter for dec2intra cmd fifo full;"]
    #[inline(always)]
    pub fn is_d12(&self) -> bool {
        *self == PerfCnt1Sel::D12
    }
    #[doc = "cycles counter for mc2recon cmd fifo empty;"]
    #[inline(always)]
    pub fn is_d13(&self) -> bool {
        *self == PerfCnt1Sel::D13
    }
    #[doc = "cycles counter for mc2recon cmd fifo full;"]
    #[inline(always)]
    pub fn is_d14(&self) -> bool {
        *self == PerfCnt1Sel::D14
    }
    #[doc = "cycles counter for mc2recon data fifo empty;"]
    #[inline(always)]
    pub fn is_d15(&self) -> bool {
        *self == PerfCnt1Sel::D15
    }
    #[doc = "cycles counter for mc2recon data fifo full;"]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == PerfCnt1Sel::D16
    }
    #[doc = "cycles counter for recon2filter data write allow;"]
    #[inline(always)]
    pub fn is_d17(&self) -> bool {
        *self == PerfCnt1Sel::D17
    }
    #[doc = "cycles counter for inter2busifd cmd fifo empty;"]
    #[inline(always)]
    pub fn is_d18(&self) -> bool {
        *self == PerfCnt1Sel::D18
    }
    #[doc = "cycles counter for inter2busifd cmd fifo full;"]
    #[inline(always)]
    pub fn is_d19(&self) -> bool {
        *self == PerfCnt1Sel::D19
    }
    #[doc = "cycles counter for busifd2mc data fifo empty;"]
    #[inline(always)]
    pub fn is_d20(&self) -> bool {
        *self == PerfCnt1Sel::D20
    }
    #[doc = "cycles counter for busifd2mc data fifo full;"]
    #[inline(always)]
    pub fn is_d21(&self) -> bool {
        *self == PerfCnt1Sel::D21
    }
    #[doc = "cycles counter for bus working status;"]
    #[inline(always)]
    pub fn is_d22(&self) -> bool {
        *self == PerfCnt1Sel::D22
    }
    #[doc = "cycles counter for dec2inter cmd fifo empty;"]
    #[inline(always)]
    pub fn is_d23(&self) -> bool {
        *self == PerfCnt1Sel::D23
    }
    #[doc = "cycles counter for dec2inter cmd fifo full;"]
    #[inline(always)]
    pub fn is_d24(&self) -> bool {
        *self == PerfCnt1Sel::D24
    }
    #[doc = "cycles counter for inter2mc cmd fifo empty;"]
    #[inline(always)]
    pub fn is_d25(&self) -> bool {
        *self == PerfCnt1Sel::D25
    }
    #[doc = "cycles counter for inter2mc cmd fifo full;"]
    #[inline(always)]
    pub fn is_d26(&self) -> bool {
        *self == PerfCnt1Sel::D26
    }
    #[doc = "cycles counter for inter2dblk bs fifo empty;"]
    #[inline(always)]
    pub fn is_d27(&self) -> bool {
        *self == PerfCnt1Sel::D27
    }
    #[doc = "cycles counter for inter2dblk bs fifo full;"]
    #[inline(always)]
    pub fn is_d28(&self) -> bool {
        *self == PerfCnt1Sel::D28
    }
    #[doc = "cycles counter for colmv_rbuf_empty;"]
    #[inline(always)]
    pub fn is_d29(&self) -> bool {
        *self == PerfCnt1Sel::D29
    }
    #[doc = "cycles counter for colmv_rbuf_full;"]
    #[inline(always)]
    pub fn is_d30(&self) -> bool {
        *self == PerfCnt1Sel::D30
    }
    #[doc = "cycles counter for colmv_wbuf_empty;"]
    #[inline(always)]
    pub fn is_d31(&self) -> bool {
        *self == PerfCnt1Sel::D31
    }
    #[doc = "cycles counter for colmv_wbuf_da_full;"]
    #[inline(always)]
    pub fn is_d32(&self) -> bool {
        *self == PerfCnt1Sel::D32
    }
    #[doc = "cycles counter for dblk input data valid;"]
    #[inline(always)]
    pub fn is_d33(&self) -> bool {
        *self == PerfCnt1Sel::D33
    }
    #[doc = "cycles counter for dblk can't write data to sao;"]
    #[inline(always)]
    pub fn is_d34(&self) -> bool {
        *self == PerfCnt1Sel::D34
    }
    #[doc = "cycles counter for dec2loopfilter cmd fifo empty;"]
    #[inline(always)]
    pub fn is_d35(&self) -> bool {
        *self == PerfCnt1Sel::D35
    }
    #[doc = "cycles counter for dec2loopfilter cmd fifo full;"]
    #[inline(always)]
    pub fn is_d36(&self) -> bool {
        *self == PerfCnt1Sel::D36
    }
    #[doc = "cycles counter for sao input data valid;"]
    #[inline(always)]
    pub fn is_d37(&self) -> bool {
        *self == PerfCnt1Sel::D37
    }
    #[doc = "cycles counter for busifd hold back sao write data;"]
    #[inline(always)]
    pub fn is_d38(&self) -> bool {
        *self == PerfCnt1Sel::D38
    }
    #[doc = "cycles counter for sao output data valid;"]
    #[inline(always)]
    pub fn is_d39(&self) -> bool {
        *self == PerfCnt1Sel::D39
    }
    #[doc = "counter for dec_ctrl read cmd num"]
    #[inline(always)]
    pub fn is_d40(&self) -> bool {
        *self == PerfCnt1Sel::D40
    }
}
#[doc = "Field `PERF_CNT1_SEL` writer - Field0000 Abstract"]
pub type PerfCnt1SelW<'a, REG> = crate::FieldWriter<'a, REG, 6, PerfCnt1Sel>;
impl<'a, REG> PerfCnt1SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "don't work;"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D0)
    }
    #[doc = "cycles counter for cabac in buffer empty;"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D1)
    }
    #[doc = "cycles counter for cabac in buffer full;"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D2)
    }
    #[doc = "cycles counter for cabac out buffer empty;"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D3)
    }
    #[doc = "cycles counter for cabac out buffer full;"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D4)
    }
    #[doc = "cycles counter for transd input data ready;"]
    #[inline(always)]
    pub fn d5(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D5)
    }
    #[doc = "cycles counter for transd write data to recon allow;"]
    #[inline(always)]
    pub fn d6(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D6)
    }
    #[doc = "cycles counter for dec2transd cmd empty;"]
    #[inline(always)]
    pub fn d7(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D7)
    }
    #[doc = "cycles counter for dec2transd cmd full;"]
    #[inline(always)]
    pub fn d8(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D8)
    }
    #[doc = "cycles counter for transd2dblk bs fifo empty;"]
    #[inline(always)]
    pub fn d9(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D9)
    }
    #[doc = "cycles counter for transd2dblk bs fifo full;"]
    #[inline(always)]
    pub fn d10(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D10)
    }
    #[doc = "cycles counter for dec2intra cmd fifo empty;"]
    #[inline(always)]
    pub fn d11(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D11)
    }
    #[doc = "cycles counter for dec2intra cmd fifo full;"]
    #[inline(always)]
    pub fn d12(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D12)
    }
    #[doc = "cycles counter for mc2recon cmd fifo empty;"]
    #[inline(always)]
    pub fn d13(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D13)
    }
    #[doc = "cycles counter for mc2recon cmd fifo full;"]
    #[inline(always)]
    pub fn d14(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D14)
    }
    #[doc = "cycles counter for mc2recon data fifo empty;"]
    #[inline(always)]
    pub fn d15(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D15)
    }
    #[doc = "cycles counter for mc2recon data fifo full;"]
    #[inline(always)]
    pub fn d16(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D16)
    }
    #[doc = "cycles counter for recon2filter data write allow;"]
    #[inline(always)]
    pub fn d17(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D17)
    }
    #[doc = "cycles counter for inter2busifd cmd fifo empty;"]
    #[inline(always)]
    pub fn d18(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D18)
    }
    #[doc = "cycles counter for inter2busifd cmd fifo full;"]
    #[inline(always)]
    pub fn d19(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D19)
    }
    #[doc = "cycles counter for busifd2mc data fifo empty;"]
    #[inline(always)]
    pub fn d20(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D20)
    }
    #[doc = "cycles counter for busifd2mc data fifo full;"]
    #[inline(always)]
    pub fn d21(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D21)
    }
    #[doc = "cycles counter for bus working status;"]
    #[inline(always)]
    pub fn d22(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D22)
    }
    #[doc = "cycles counter for dec2inter cmd fifo empty;"]
    #[inline(always)]
    pub fn d23(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D23)
    }
    #[doc = "cycles counter for dec2inter cmd fifo full;"]
    #[inline(always)]
    pub fn d24(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D24)
    }
    #[doc = "cycles counter for inter2mc cmd fifo empty;"]
    #[inline(always)]
    pub fn d25(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D25)
    }
    #[doc = "cycles counter for inter2mc cmd fifo full;"]
    #[inline(always)]
    pub fn d26(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D26)
    }
    #[doc = "cycles counter for inter2dblk bs fifo empty;"]
    #[inline(always)]
    pub fn d27(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D27)
    }
    #[doc = "cycles counter for inter2dblk bs fifo full;"]
    #[inline(always)]
    pub fn d28(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D28)
    }
    #[doc = "cycles counter for colmv_rbuf_empty;"]
    #[inline(always)]
    pub fn d29(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D29)
    }
    #[doc = "cycles counter for colmv_rbuf_full;"]
    #[inline(always)]
    pub fn d30(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D30)
    }
    #[doc = "cycles counter for colmv_wbuf_empty;"]
    #[inline(always)]
    pub fn d31(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D31)
    }
    #[doc = "cycles counter for colmv_wbuf_da_full;"]
    #[inline(always)]
    pub fn d32(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D32)
    }
    #[doc = "cycles counter for dblk input data valid;"]
    #[inline(always)]
    pub fn d33(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D33)
    }
    #[doc = "cycles counter for dblk can't write data to sao;"]
    #[inline(always)]
    pub fn d34(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D34)
    }
    #[doc = "cycles counter for dec2loopfilter cmd fifo empty;"]
    #[inline(always)]
    pub fn d35(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D35)
    }
    #[doc = "cycles counter for dec2loopfilter cmd fifo full;"]
    #[inline(always)]
    pub fn d36(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D36)
    }
    #[doc = "cycles counter for sao input data valid;"]
    #[inline(always)]
    pub fn d37(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D37)
    }
    #[doc = "cycles counter for busifd hold back sao write data;"]
    #[inline(always)]
    pub fn d38(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D38)
    }
    #[doc = "cycles counter for sao output data valid;"]
    #[inline(always)]
    pub fn d39(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D39)
    }
    #[doc = "counter for dec_ctrl read cmd num"]
    #[inline(always)]
    pub fn d40(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt1Sel::D40)
    }
}
#[doc = "Field0000 Abstract\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PerfCnt2Sel {
    #[doc = "0: don't work;"]
    D0 = 0,
    #[doc = "1: cycles counter for cabac in buffer empty;"]
    D1 = 1,
    #[doc = "2: cycles counter for cabac in buffer full;"]
    D2 = 2,
    #[doc = "3: cycles counter for cabac out buffer empty;"]
    D3 = 3,
    #[doc = "4: cycles counter for cabac out buffer full;"]
    D4 = 4,
    #[doc = "5: cycles counter for transd input data ready;"]
    D5 = 5,
    #[doc = "6: cycles counter for transd write data to recon allow;"]
    D6 = 6,
    #[doc = "7: cycles counter for dec2transd cmd empty;"]
    D7 = 7,
    #[doc = "8: cycles counter for dec2transd cmd full;"]
    D8 = 8,
    #[doc = "9: cycles counter for transd2dblk bs fifo empty;"]
    D9 = 9,
    #[doc = "10: cycles counter for transd2dblk bs fifo full;"]
    D10 = 10,
    #[doc = "11: cycles counter for dec2intra cmd fifo empty;"]
    D11 = 11,
    #[doc = "12: cycles counter for dec2intra cmd fifo full;"]
    D12 = 12,
    #[doc = "13: cycles counter for mc2recon cmd fifo empty;"]
    D13 = 13,
    #[doc = "14: cycles counter for mc2recon cmd fifo full;"]
    D14 = 14,
    #[doc = "15: cycles counter for mc2recon data fifo empty;"]
    D15 = 15,
    #[doc = "16: cycles counter for mc2recon data fifo full;"]
    D16 = 16,
    #[doc = "17: cycles counter for recon2filter data write allow;"]
    D17 = 17,
    #[doc = "18: cycles counter for inter2busifd cmd fifo empty;"]
    D18 = 18,
    #[doc = "19: cycles counter for inter2busifd cmd fifo full;"]
    D19 = 19,
    #[doc = "20: cycles counter for busifd2mc data fifo empty;"]
    D20 = 20,
    #[doc = "21: cycles counter for busifd2mc data fifo full;"]
    D21 = 21,
    #[doc = "22: cycles counter for bus working status;"]
    D22 = 22,
    #[doc = "23: cycles counter for dec2inter cmd fifo empty;"]
    D23 = 23,
    #[doc = "24: cycles counter for dec2inter cmd fifo full;"]
    D24 = 24,
    #[doc = "25: cycles counter for inter2mc cmd fifo empty;"]
    D25 = 25,
    #[doc = "26: cycles counter for inter2mc cmd fifo full;"]
    D26 = 26,
    #[doc = "27: cycles counter for inter2dblk bs fifo empty;"]
    D27 = 27,
    #[doc = "28: cycles counter for inter2dblk bs fifo full;"]
    D28 = 28,
    #[doc = "29: cycles counter for colmv_rbuf_empty;"]
    D29 = 29,
    #[doc = "30: cycles counter for colmv_rbuf_full;"]
    D30 = 30,
    #[doc = "31: cycles counter for colmv_wbuf_empty;"]
    D31 = 31,
    #[doc = "32: cycles counter for colmv_wbuf_da_full;"]
    D32 = 32,
    #[doc = "33: cycles counter for dblk input data valid;"]
    D33 = 33,
    #[doc = "34: cycles counter for dblk can't write data to sao;"]
    D34 = 34,
    #[doc = "35: cycles counter for dec2loopfilter cmd fifo empty;"]
    D35 = 35,
    #[doc = "36: cycles counter for dec2loopfilter cmd fifo full;"]
    D36 = 36,
    #[doc = "37: cycles counter for sao input data valid;"]
    D37 = 37,
    #[doc = "38: cycles counter for busifd hold back sao write data;"]
    D38 = 38,
    #[doc = "39: cycles counter for sao output data valid;"]
    D39 = 39,
    #[doc = "40: counter for dec_ctrl read cmd num"]
    D40 = 40,
}
impl From<PerfCnt2Sel> for u8 {
    #[inline(always)]
    fn from(variant: PerfCnt2Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PerfCnt2Sel {
    type Ux = u8;
}
#[doc = "Field `PERF_CNT2_SEL` reader - Field0000 Abstract"]
pub type PerfCnt2SelR = crate::FieldReader<PerfCnt2Sel>;
impl PerfCnt2SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PerfCnt2Sel> {
        match self.bits {
            0 => Some(PerfCnt2Sel::D0),
            1 => Some(PerfCnt2Sel::D1),
            2 => Some(PerfCnt2Sel::D2),
            3 => Some(PerfCnt2Sel::D3),
            4 => Some(PerfCnt2Sel::D4),
            5 => Some(PerfCnt2Sel::D5),
            6 => Some(PerfCnt2Sel::D6),
            7 => Some(PerfCnt2Sel::D7),
            8 => Some(PerfCnt2Sel::D8),
            9 => Some(PerfCnt2Sel::D9),
            10 => Some(PerfCnt2Sel::D10),
            11 => Some(PerfCnt2Sel::D11),
            12 => Some(PerfCnt2Sel::D12),
            13 => Some(PerfCnt2Sel::D13),
            14 => Some(PerfCnt2Sel::D14),
            15 => Some(PerfCnt2Sel::D15),
            16 => Some(PerfCnt2Sel::D16),
            17 => Some(PerfCnt2Sel::D17),
            18 => Some(PerfCnt2Sel::D18),
            19 => Some(PerfCnt2Sel::D19),
            20 => Some(PerfCnt2Sel::D20),
            21 => Some(PerfCnt2Sel::D21),
            22 => Some(PerfCnt2Sel::D22),
            23 => Some(PerfCnt2Sel::D23),
            24 => Some(PerfCnt2Sel::D24),
            25 => Some(PerfCnt2Sel::D25),
            26 => Some(PerfCnt2Sel::D26),
            27 => Some(PerfCnt2Sel::D27),
            28 => Some(PerfCnt2Sel::D28),
            29 => Some(PerfCnt2Sel::D29),
            30 => Some(PerfCnt2Sel::D30),
            31 => Some(PerfCnt2Sel::D31),
            32 => Some(PerfCnt2Sel::D32),
            33 => Some(PerfCnt2Sel::D33),
            34 => Some(PerfCnt2Sel::D34),
            35 => Some(PerfCnt2Sel::D35),
            36 => Some(PerfCnt2Sel::D36),
            37 => Some(PerfCnt2Sel::D37),
            38 => Some(PerfCnt2Sel::D38),
            39 => Some(PerfCnt2Sel::D39),
            40 => Some(PerfCnt2Sel::D40),
            _ => None,
        }
    }
    #[doc = "don't work;"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == PerfCnt2Sel::D0
    }
    #[doc = "cycles counter for cabac in buffer empty;"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == PerfCnt2Sel::D1
    }
    #[doc = "cycles counter for cabac in buffer full;"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == PerfCnt2Sel::D2
    }
    #[doc = "cycles counter for cabac out buffer empty;"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == PerfCnt2Sel::D3
    }
    #[doc = "cycles counter for cabac out buffer full;"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == PerfCnt2Sel::D4
    }
    #[doc = "cycles counter for transd input data ready;"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == PerfCnt2Sel::D5
    }
    #[doc = "cycles counter for transd write data to recon allow;"]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == PerfCnt2Sel::D6
    }
    #[doc = "cycles counter for dec2transd cmd empty;"]
    #[inline(always)]
    pub fn is_d7(&self) -> bool {
        *self == PerfCnt2Sel::D7
    }
    #[doc = "cycles counter for dec2transd cmd full;"]
    #[inline(always)]
    pub fn is_d8(&self) -> bool {
        *self == PerfCnt2Sel::D8
    }
    #[doc = "cycles counter for transd2dblk bs fifo empty;"]
    #[inline(always)]
    pub fn is_d9(&self) -> bool {
        *self == PerfCnt2Sel::D9
    }
    #[doc = "cycles counter for transd2dblk bs fifo full;"]
    #[inline(always)]
    pub fn is_d10(&self) -> bool {
        *self == PerfCnt2Sel::D10
    }
    #[doc = "cycles counter for dec2intra cmd fifo empty;"]
    #[inline(always)]
    pub fn is_d11(&self) -> bool {
        *self == PerfCnt2Sel::D11
    }
    #[doc = "cycles counter for dec2intra cmd fifo full;"]
    #[inline(always)]
    pub fn is_d12(&self) -> bool {
        *self == PerfCnt2Sel::D12
    }
    #[doc = "cycles counter for mc2recon cmd fifo empty;"]
    #[inline(always)]
    pub fn is_d13(&self) -> bool {
        *self == PerfCnt2Sel::D13
    }
    #[doc = "cycles counter for mc2recon cmd fifo full;"]
    #[inline(always)]
    pub fn is_d14(&self) -> bool {
        *self == PerfCnt2Sel::D14
    }
    #[doc = "cycles counter for mc2recon data fifo empty;"]
    #[inline(always)]
    pub fn is_d15(&self) -> bool {
        *self == PerfCnt2Sel::D15
    }
    #[doc = "cycles counter for mc2recon data fifo full;"]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == PerfCnt2Sel::D16
    }
    #[doc = "cycles counter for recon2filter data write allow;"]
    #[inline(always)]
    pub fn is_d17(&self) -> bool {
        *self == PerfCnt2Sel::D17
    }
    #[doc = "cycles counter for inter2busifd cmd fifo empty;"]
    #[inline(always)]
    pub fn is_d18(&self) -> bool {
        *self == PerfCnt2Sel::D18
    }
    #[doc = "cycles counter for inter2busifd cmd fifo full;"]
    #[inline(always)]
    pub fn is_d19(&self) -> bool {
        *self == PerfCnt2Sel::D19
    }
    #[doc = "cycles counter for busifd2mc data fifo empty;"]
    #[inline(always)]
    pub fn is_d20(&self) -> bool {
        *self == PerfCnt2Sel::D20
    }
    #[doc = "cycles counter for busifd2mc data fifo full;"]
    #[inline(always)]
    pub fn is_d21(&self) -> bool {
        *self == PerfCnt2Sel::D21
    }
    #[doc = "cycles counter for bus working status;"]
    #[inline(always)]
    pub fn is_d22(&self) -> bool {
        *self == PerfCnt2Sel::D22
    }
    #[doc = "cycles counter for dec2inter cmd fifo empty;"]
    #[inline(always)]
    pub fn is_d23(&self) -> bool {
        *self == PerfCnt2Sel::D23
    }
    #[doc = "cycles counter for dec2inter cmd fifo full;"]
    #[inline(always)]
    pub fn is_d24(&self) -> bool {
        *self == PerfCnt2Sel::D24
    }
    #[doc = "cycles counter for inter2mc cmd fifo empty;"]
    #[inline(always)]
    pub fn is_d25(&self) -> bool {
        *self == PerfCnt2Sel::D25
    }
    #[doc = "cycles counter for inter2mc cmd fifo full;"]
    #[inline(always)]
    pub fn is_d26(&self) -> bool {
        *self == PerfCnt2Sel::D26
    }
    #[doc = "cycles counter for inter2dblk bs fifo empty;"]
    #[inline(always)]
    pub fn is_d27(&self) -> bool {
        *self == PerfCnt2Sel::D27
    }
    #[doc = "cycles counter for inter2dblk bs fifo full;"]
    #[inline(always)]
    pub fn is_d28(&self) -> bool {
        *self == PerfCnt2Sel::D28
    }
    #[doc = "cycles counter for colmv_rbuf_empty;"]
    #[inline(always)]
    pub fn is_d29(&self) -> bool {
        *self == PerfCnt2Sel::D29
    }
    #[doc = "cycles counter for colmv_rbuf_full;"]
    #[inline(always)]
    pub fn is_d30(&self) -> bool {
        *self == PerfCnt2Sel::D30
    }
    #[doc = "cycles counter for colmv_wbuf_empty;"]
    #[inline(always)]
    pub fn is_d31(&self) -> bool {
        *self == PerfCnt2Sel::D31
    }
    #[doc = "cycles counter for colmv_wbuf_da_full;"]
    #[inline(always)]
    pub fn is_d32(&self) -> bool {
        *self == PerfCnt2Sel::D32
    }
    #[doc = "cycles counter for dblk input data valid;"]
    #[inline(always)]
    pub fn is_d33(&self) -> bool {
        *self == PerfCnt2Sel::D33
    }
    #[doc = "cycles counter for dblk can't write data to sao;"]
    #[inline(always)]
    pub fn is_d34(&self) -> bool {
        *self == PerfCnt2Sel::D34
    }
    #[doc = "cycles counter for dec2loopfilter cmd fifo empty;"]
    #[inline(always)]
    pub fn is_d35(&self) -> bool {
        *self == PerfCnt2Sel::D35
    }
    #[doc = "cycles counter for dec2loopfilter cmd fifo full;"]
    #[inline(always)]
    pub fn is_d36(&self) -> bool {
        *self == PerfCnt2Sel::D36
    }
    #[doc = "cycles counter for sao input data valid;"]
    #[inline(always)]
    pub fn is_d37(&self) -> bool {
        *self == PerfCnt2Sel::D37
    }
    #[doc = "cycles counter for busifd hold back sao write data;"]
    #[inline(always)]
    pub fn is_d38(&self) -> bool {
        *self == PerfCnt2Sel::D38
    }
    #[doc = "cycles counter for sao output data valid;"]
    #[inline(always)]
    pub fn is_d39(&self) -> bool {
        *self == PerfCnt2Sel::D39
    }
    #[doc = "counter for dec_ctrl read cmd num"]
    #[inline(always)]
    pub fn is_d40(&self) -> bool {
        *self == PerfCnt2Sel::D40
    }
}
#[doc = "Field `PERF_CNT2_SEL` writer - Field0000 Abstract"]
pub type PerfCnt2SelW<'a, REG> = crate::FieldWriter<'a, REG, 6, PerfCnt2Sel>;
impl<'a, REG> PerfCnt2SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "don't work;"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D0)
    }
    #[doc = "cycles counter for cabac in buffer empty;"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D1)
    }
    #[doc = "cycles counter for cabac in buffer full;"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D2)
    }
    #[doc = "cycles counter for cabac out buffer empty;"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D3)
    }
    #[doc = "cycles counter for cabac out buffer full;"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D4)
    }
    #[doc = "cycles counter for transd input data ready;"]
    #[inline(always)]
    pub fn d5(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D5)
    }
    #[doc = "cycles counter for transd write data to recon allow;"]
    #[inline(always)]
    pub fn d6(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D6)
    }
    #[doc = "cycles counter for dec2transd cmd empty;"]
    #[inline(always)]
    pub fn d7(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D7)
    }
    #[doc = "cycles counter for dec2transd cmd full;"]
    #[inline(always)]
    pub fn d8(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D8)
    }
    #[doc = "cycles counter for transd2dblk bs fifo empty;"]
    #[inline(always)]
    pub fn d9(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D9)
    }
    #[doc = "cycles counter for transd2dblk bs fifo full;"]
    #[inline(always)]
    pub fn d10(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D10)
    }
    #[doc = "cycles counter for dec2intra cmd fifo empty;"]
    #[inline(always)]
    pub fn d11(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D11)
    }
    #[doc = "cycles counter for dec2intra cmd fifo full;"]
    #[inline(always)]
    pub fn d12(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D12)
    }
    #[doc = "cycles counter for mc2recon cmd fifo empty;"]
    #[inline(always)]
    pub fn d13(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D13)
    }
    #[doc = "cycles counter for mc2recon cmd fifo full;"]
    #[inline(always)]
    pub fn d14(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D14)
    }
    #[doc = "cycles counter for mc2recon data fifo empty;"]
    #[inline(always)]
    pub fn d15(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D15)
    }
    #[doc = "cycles counter for mc2recon data fifo full;"]
    #[inline(always)]
    pub fn d16(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D16)
    }
    #[doc = "cycles counter for recon2filter data write allow;"]
    #[inline(always)]
    pub fn d17(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D17)
    }
    #[doc = "cycles counter for inter2busifd cmd fifo empty;"]
    #[inline(always)]
    pub fn d18(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D18)
    }
    #[doc = "cycles counter for inter2busifd cmd fifo full;"]
    #[inline(always)]
    pub fn d19(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D19)
    }
    #[doc = "cycles counter for busifd2mc data fifo empty;"]
    #[inline(always)]
    pub fn d20(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D20)
    }
    #[doc = "cycles counter for busifd2mc data fifo full;"]
    #[inline(always)]
    pub fn d21(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D21)
    }
    #[doc = "cycles counter for bus working status;"]
    #[inline(always)]
    pub fn d22(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D22)
    }
    #[doc = "cycles counter for dec2inter cmd fifo empty;"]
    #[inline(always)]
    pub fn d23(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D23)
    }
    #[doc = "cycles counter for dec2inter cmd fifo full;"]
    #[inline(always)]
    pub fn d24(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D24)
    }
    #[doc = "cycles counter for inter2mc cmd fifo empty;"]
    #[inline(always)]
    pub fn d25(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D25)
    }
    #[doc = "cycles counter for inter2mc cmd fifo full;"]
    #[inline(always)]
    pub fn d26(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D26)
    }
    #[doc = "cycles counter for inter2dblk bs fifo empty;"]
    #[inline(always)]
    pub fn d27(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D27)
    }
    #[doc = "cycles counter for inter2dblk bs fifo full;"]
    #[inline(always)]
    pub fn d28(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D28)
    }
    #[doc = "cycles counter for colmv_rbuf_empty;"]
    #[inline(always)]
    pub fn d29(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D29)
    }
    #[doc = "cycles counter for colmv_rbuf_full;"]
    #[inline(always)]
    pub fn d30(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D30)
    }
    #[doc = "cycles counter for colmv_wbuf_empty;"]
    #[inline(always)]
    pub fn d31(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D31)
    }
    #[doc = "cycles counter for colmv_wbuf_da_full;"]
    #[inline(always)]
    pub fn d32(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D32)
    }
    #[doc = "cycles counter for dblk input data valid;"]
    #[inline(always)]
    pub fn d33(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D33)
    }
    #[doc = "cycles counter for dblk can't write data to sao;"]
    #[inline(always)]
    pub fn d34(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D34)
    }
    #[doc = "cycles counter for dec2loopfilter cmd fifo empty;"]
    #[inline(always)]
    pub fn d35(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D35)
    }
    #[doc = "cycles counter for dec2loopfilter cmd fifo full;"]
    #[inline(always)]
    pub fn d36(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D36)
    }
    #[doc = "cycles counter for sao input data valid;"]
    #[inline(always)]
    pub fn d37(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D37)
    }
    #[doc = "cycles counter for busifd hold back sao write data;"]
    #[inline(always)]
    pub fn d38(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D38)
    }
    #[doc = "cycles counter for sao output data valid;"]
    #[inline(always)]
    pub fn d39(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D39)
    }
    #[doc = "counter for dec_ctrl read cmd num"]
    #[inline(always)]
    pub fn d40(self) -> &'a mut crate::W<REG> {
        self.variant(PerfCnt2Sel::D40)
    }
}
impl R {
    #[doc = "Bits 0:5 - sel counter0 to cal which signal"]
    #[inline(always)]
    pub fn perf_cnt0_sel(&self) -> PerfCnt0SelR {
        PerfCnt0SelR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Field0000 Abstract"]
    #[inline(always)]
    pub fn perf_cnt1_sel(&self) -> PerfCnt1SelR {
        PerfCnt1SelR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Field0000 Abstract"]
    #[inline(always)]
    pub fn perf_cnt2_sel(&self) -> PerfCnt2SelR {
        PerfCnt2SelR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - sel counter0 to cal which signal"]
    #[inline(always)]
    #[must_use]
    pub fn perf_cnt0_sel(&mut self) -> PerfCnt0SelW<Swreg68PerformanceSelSpec> {
        PerfCnt0SelW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Field0000 Abstract"]
    #[inline(always)]
    #[must_use]
    pub fn perf_cnt1_sel(&mut self) -> PerfCnt1SelW<Swreg68PerformanceSelSpec> {
        PerfCnt1SelW::new(self, 8)
    }
    #[doc = "Bits 16:21 - Field0000 Abstract"]
    #[inline(always)]
    #[must_use]
    pub fn perf_cnt2_sel(&mut self) -> PerfCnt2SelW<Swreg68PerformanceSelSpec> {
        PerfCnt2SelW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg68_performance_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg68_performance_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg68PerformanceSelSpec;
impl crate::RegisterSpec for Swreg68PerformanceSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg68_performance_sel::R`](R) reader structure"]
impl crate::Readable for Swreg68PerformanceSelSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg68_performance_sel::W`](W) writer structure"]
impl crate::Writable for Swreg68PerformanceSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG68_PERFORMANCE_SEL to value 0"]
impl crate::Resettable for Swreg68PerformanceSelSpec {
    const RESET_VALUE: u32 = 0;
}
