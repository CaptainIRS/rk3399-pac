#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0xc100],
    gsbuscfg0: Gsbuscfg0,
    gsbuscfg1: Gsbuscfg1,
    gtxthrcfg: Gtxthrcfg,
    grxthrcfg: Grxthrcfg,
    gctl: Gctl,
    gpmsts: Gpmsts,
    gsts: Gsts,
    guctl1: Guctl1,
    gsnpsid: Gsnpsid,
    ggpio: Ggpio,
    guid: Guid,
    guctl: Guctl,
    gbuserraddrlo: Gbuserraddrlo,
    gbuserraddrhi: Gbuserraddrhi,
    gprtbimaplo: Gprtbimaplo,
    _reserved15: [u8; 0x04],
    ghwparams0: Ghwparams0,
    ghwparams1: Ghwparams1,
    ghwparams2: Ghwparams2,
    ghwparams3: Ghwparams3,
    ghwparams4: Ghwparams4,
    ghwparams5: Ghwparams5,
    ghwparams6: Ghwparams6,
    ghwparams7: Ghwparams7,
    gdbgfifospace: Gdbgfifospace,
    gdbgltssm: Gdbgltssm,
    gdbglnmcc: Gdbglnmcc,
    gdbgbmu: Gdbgbmu,
    gdbglspmux: Gdbglspmux,
    gdbglsp: Gdbglsp,
    gdbgepinfo0: Gdbgepinfo0,
    gdbgepinfo1: Gdbgepinfo1,
    gprtbimap_hslo: GprtbimapHslo,
    _reserved32: [u8; 0x04],
    gprtbimap_fslo: GprtbimapFslo,
    _reserved33: [u8; 0x74],
    gusb2phycfg0: Gusb2phycfg0,
    _reserved34: [u8; 0xbc],
    gusb3pipectl0: Gusb3pipectl0,
    _reserved35: [u8; 0x3c],
    gtxfifosiz: [Gtxfifosiz; 7],
    _reserved36: [u8; 0x64],
    grxfifosiz: [Grxfifosiz; 3],
    _reserved37: [u8; 0x74],
    gevntadrlo0: Gevntadrlo0,
    gevntadrhi0: Gevntadrhi0,
    gevntsiz0: Gevntsiz0,
    gevntcount0: Gevntcount0,
    _reserved41: [u8; 0x01f0],
    ghwparams8: Ghwparams8,
    _reserved42: [u8; 0x0c],
    gtxfifopridev: Gtxfifopridev,
    _reserved43: [u8; 0x04],
    gtxfifoprihst: Gtxfifoprihst,
    grxfifoprihst: Grxfifoprihst,
    gfifopridbc: Gfifopridbc,
    gdmahlratio: Gdmahlratio,
    _reserved47: [u8; 0x08],
    gfladj: Gfladj,
    _reserved48: [u8; 0xcc],
    dcfg: Dcfg,
    dctl: Dctl,
    devten: Devten,
    dsts: Dsts,
    dgcmdpar: Dgcmdpar,
    dgcmd: Dgcmd,
    _reserved54: [u8; 0x08],
    dalepena: Dalepena,
    _reserved55: [u8; 0xdc],
    depcmdpar2: (),
    _reserved56: [u8; 0x04],
    depcmdpar1: (),
    _reserved57: [u8; 0x04],
    depcmdpar0: (),
    _reserved58: [u8; 0x04],
    depcmd: (),
}
impl RegisterBlock {
    #[doc = "0xc100 - Global SoC Bus Configuration Register 0"]
    #[inline(always)]
    pub const fn gsbuscfg0(&self) -> &Gsbuscfg0 {
        &self.gsbuscfg0
    }
    #[doc = "0xc104 - Global SoC Bus Configuration Register 1"]
    #[inline(always)]
    pub const fn gsbuscfg1(&self) -> &Gsbuscfg1 {
        &self.gsbuscfg1
    }
    #[doc = "0xc108 - Global Tx Threshold Control Register"]
    #[inline(always)]
    pub const fn gtxthrcfg(&self) -> &Gtxthrcfg {
        &self.gtxthrcfg
    }
    #[doc = "0xc10c - Global Rx Threshold Control Register"]
    #[inline(always)]
    pub const fn grxthrcfg(&self) -> &Grxthrcfg {
        &self.grxthrcfg
    }
    #[doc = "0xc110 - Global Core Control Register"]
    #[inline(always)]
    pub const fn gctl(&self) -> &Gctl {
        &self.gctl
    }
    #[doc = "0xc114 - Global Power Management Status Register"]
    #[inline(always)]
    pub const fn gpmsts(&self) -> &Gpmsts {
        &self.gpmsts
    }
    #[doc = "0xc118 - Global Status Register"]
    #[inline(always)]
    pub const fn gsts(&self) -> &Gsts {
        &self.gsts
    }
    #[doc = "0xc11c - Global User Control Register 1"]
    #[inline(always)]
    pub const fn guctl1(&self) -> &Guctl1 {
        &self.guctl1
    }
    #[doc = "0xc120 - Global SNPS ID Register"]
    #[inline(always)]
    pub const fn gsnpsid(&self) -> &Gsnpsid {
        &self.gsnpsid
    }
    #[doc = "0xc124 - Global General Purpose Input/Output Register"]
    #[inline(always)]
    pub const fn ggpio(&self) -> &Ggpio {
        &self.ggpio
    }
    #[doc = "0xc128 - Global User ID Register"]
    #[inline(always)]
    pub const fn guid(&self) -> &Guid {
        &self.guid
    }
    #[doc = "0xc12c - Global User Control Register"]
    #[inline(always)]
    pub const fn guctl(&self) -> &Guctl {
        &self.guctl
    }
    #[doc = "0xc130 - Global SoC Bus Error Address Register - Low"]
    #[inline(always)]
    pub const fn gbuserraddrlo(&self) -> &Gbuserraddrlo {
        &self.gbuserraddrlo
    }
    #[doc = "0xc134 - Global SoC Bus Error Address Register - High"]
    #[inline(always)]
    pub const fn gbuserraddrhi(&self) -> &Gbuserraddrhi {
        &self.gbuserraddrhi
    }
    #[doc = "0xc138 - Global SS Port to Bus Instance Mapping Register - Low"]
    #[inline(always)]
    pub const fn gprtbimaplo(&self) -> &Gprtbimaplo {
        &self.gprtbimaplo
    }
    #[doc = "0xc140 - Global Hardware Parameters Register 0"]
    #[inline(always)]
    pub const fn ghwparams0(&self) -> &Ghwparams0 {
        &self.ghwparams0
    }
    #[doc = "0xc144 - Global Hardware Parameters Register 1"]
    #[inline(always)]
    pub const fn ghwparams1(&self) -> &Ghwparams1 {
        &self.ghwparams1
    }
    #[doc = "0xc148 - Global Hardware Parameters Register 2"]
    #[inline(always)]
    pub const fn ghwparams2(&self) -> &Ghwparams2 {
        &self.ghwparams2
    }
    #[doc = "0xc14c - Global Hardware Parameters Register 3"]
    #[inline(always)]
    pub const fn ghwparams3(&self) -> &Ghwparams3 {
        &self.ghwparams3
    }
    #[doc = "0xc150 - Global Hardware Parameters Register 4"]
    #[inline(always)]
    pub const fn ghwparams4(&self) -> &Ghwparams4 {
        &self.ghwparams4
    }
    #[doc = "0xc154 - Global Hardware Parameters Register 5"]
    #[inline(always)]
    pub const fn ghwparams5(&self) -> &Ghwparams5 {
        &self.ghwparams5
    }
    #[doc = "0xc158 - Global Hardware Parameters Register 6"]
    #[inline(always)]
    pub const fn ghwparams6(&self) -> &Ghwparams6 {
        &self.ghwparams6
    }
    #[doc = "0xc15c - Global Hardware Parameters Register 7"]
    #[inline(always)]
    pub const fn ghwparams7(&self) -> &Ghwparams7 {
        &self.ghwparams7
    }
    #[doc = "0xc160 - Global Debug Queue/FIFO Space Available Register"]
    #[inline(always)]
    pub const fn gdbgfifospace(&self) -> &Gdbgfifospace {
        &self.gdbgfifospace
    }
    #[doc = "0xc164 - Global Debug LTSSM Register"]
    #[inline(always)]
    pub const fn gdbgltssm(&self) -> &Gdbgltssm {
        &self.gdbgltssm
    }
    #[doc = "0xc168 - Global Debug LNMCC Register"]
    #[inline(always)]
    pub const fn gdbglnmcc(&self) -> &Gdbglnmcc {
        &self.gdbglnmcc
    }
    #[doc = "0xc16c - Global Debug BMU Register"]
    #[inline(always)]
    pub const fn gdbgbmu(&self) -> &Gdbgbmu {
        &self.gdbgbmu
    }
    #[doc = "0xc170 - Global Debug LSP MUX Register - Device"]
    #[inline(always)]
    pub const fn gdbglspmux(&self) -> &Gdbglspmux {
        &self.gdbglspmux
    }
    #[doc = "0xc174 - Global Debug LSP Register"]
    #[inline(always)]
    pub const fn gdbglsp(&self) -> &Gdbglsp {
        &self.gdbglsp
    }
    #[doc = "0xc178 - Global Debug Endpoint Information Register 0"]
    #[inline(always)]
    pub const fn gdbgepinfo0(&self) -> &Gdbgepinfo0 {
        &self.gdbgepinfo0
    }
    #[doc = "0xc17c - Global Debug Endpoint Information Register 1"]
    #[inline(always)]
    pub const fn gdbgepinfo1(&self) -> &Gdbgepinfo1 {
        &self.gdbgepinfo1
    }
    #[doc = "0xc180 - Global High-Speed Port to Bus Instance Mapping Register - Low"]
    #[inline(always)]
    pub const fn gprtbimap_hslo(&self) -> &GprtbimapHslo {
        &self.gprtbimap_hslo
    }
    #[doc = "0xc188 - Global Full-Speed Port to Bus Instance Mapping Register - Low"]
    #[inline(always)]
    pub const fn gprtbimap_fslo(&self) -> &GprtbimapFslo {
        &self.gprtbimap_fslo
    }
    #[doc = "0xc200 - Global USB2 PHY Configuration Register 0"]
    #[inline(always)]
    pub const fn gusb2phycfg0(&self) -> &Gusb2phycfg0 {
        &self.gusb2phycfg0
    }
    #[doc = "0xc2c0 - Global USB3 PIPE Control Register 0"]
    #[inline(always)]
    pub const fn gusb3pipectl0(&self) -> &Gusb3pipectl0 {
        &self.gusb3pipectl0
    }
    #[doc = "0xc300..0xc31c - Global Transmit FIFO Size Register n"]
    #[inline(always)]
    pub const fn gtxfifosiz(&self, n: usize) -> &Gtxfifosiz {
        &self.gtxfifosiz[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc300..0xc31c - Global Transmit FIFO Size Register n"]
    #[inline(always)]
    pub fn gtxfifosiz_iter(&self) -> impl Iterator<Item = &Gtxfifosiz> {
        self.gtxfifosiz.iter()
    }
    #[doc = "0xc380..0xc38c - Global Receive FIFO Size Register n"]
    #[inline(always)]
    pub const fn grxfifosiz(&self, n: usize) -> &Grxfifosiz {
        &self.grxfifosiz[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc380..0xc38c - Global Receive FIFO Size Register n"]
    #[inline(always)]
    pub fn grxfifosiz_iter(&self) -> impl Iterator<Item = &Grxfifosiz> {
        self.grxfifosiz.iter()
    }
    #[doc = "0xc400 - Global Event Buffer Address (Low) Register 0"]
    #[inline(always)]
    pub const fn gevntadrlo0(&self) -> &Gevntadrlo0 {
        &self.gevntadrlo0
    }
    #[doc = "0xc404 - Global Event Buffer Address (High) Register 0"]
    #[inline(always)]
    pub const fn gevntadrhi0(&self) -> &Gevntadrhi0 {
        &self.gevntadrhi0
    }
    #[doc = "0xc408 - Global Event Buffer Size Register 0"]
    #[inline(always)]
    pub const fn gevntsiz0(&self) -> &Gevntsiz0 {
        &self.gevntsiz0
    }
    #[doc = "0xc40c - Global Event Buffer Count Register 0"]
    #[inline(always)]
    pub const fn gevntcount0(&self) -> &Gevntcount0 {
        &self.gevntcount0
    }
    #[doc = "0xc600 - Global Hardware Parameters Register 8"]
    #[inline(always)]
    pub const fn ghwparams8(&self) -> &Ghwparams8 {
        &self.ghwparams8
    }
    #[doc = "0xc610 - Global Device TX FIFO DMA Priority Register"]
    #[inline(always)]
    pub const fn gtxfifopridev(&self) -> &Gtxfifopridev {
        &self.gtxfifopridev
    }
    #[doc = "0xc618 - Global Host TX FIFO DMA Priority Register"]
    #[inline(always)]
    pub const fn gtxfifoprihst(&self) -> &Gtxfifoprihst {
        &self.gtxfifoprihst
    }
    #[doc = "0xc61c - Global Host RX FIFO DMA Priority Register"]
    #[inline(always)]
    pub const fn grxfifoprihst(&self) -> &Grxfifoprihst {
        &self.grxfifoprihst
    }
    #[doc = "0xc620 - Global Host Debug Capability DMA Priority Register"]
    #[inline(always)]
    pub const fn gfifopridbc(&self) -> &Gfifopridbc {
        &self.gfifopridbc
    }
    #[doc = "0xc624 - Global Host FIFO DMA High-Low Priority Ratio Register"]
    #[inline(always)]
    pub const fn gdmahlratio(&self) -> &Gdmahlratio {
        &self.gdmahlratio
    }
    #[doc = "0xc630 - Global Frame Length Adjustment Register"]
    #[inline(always)]
    pub const fn gfladj(&self) -> &Gfladj {
        &self.gfladj
    }
    #[doc = "0xc700 - Device Configuration Register"]
    #[inline(always)]
    pub const fn dcfg(&self) -> &Dcfg {
        &self.dcfg
    }
    #[doc = "0xc704 - Device Control Register"]
    #[inline(always)]
    pub const fn dctl(&self) -> &Dctl {
        &self.dctl
    }
    #[doc = "0xc708 - Device Event Enable Register"]
    #[inline(always)]
    pub const fn devten(&self) -> &Devten {
        &self.devten
    }
    #[doc = "0xc70c - Device Status Register"]
    #[inline(always)]
    pub const fn dsts(&self) -> &Dsts {
        &self.dsts
    }
    #[doc = "0xc710 - Device Generic Command Parameter Register"]
    #[inline(always)]
    pub const fn dgcmdpar(&self) -> &Dgcmdpar {
        &self.dgcmdpar
    }
    #[doc = "0xc714 - Device Generic Command Register"]
    #[inline(always)]
    pub const fn dgcmd(&self) -> &Dgcmd {
        &self.dgcmd
    }
    #[doc = "0xc720 - Device Active USB Endpoint Enable Register"]
    #[inline(always)]
    pub const fn dalepena(&self) -> &Dalepena {
        &self.dalepena
    }
    #[doc = "0xc800..0xc834 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn depcmdpar2(&self, n: usize) -> &Depcmdpar2 {
        #[allow(clippy::no_effect)]
        [(); 13][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(51200)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc800..0xc834 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub fn depcmdpar2_iter(&self) -> impl Iterator<Item = &Depcmdpar2> {
        (0..13).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(51200)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0xc800 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn dep0cmdpar2(&self) -> &Depcmdpar2 {
        self.depcmdpar2(0)
    }
    #[doc = "0xc810 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn dep1cmdpar2(&self) -> &Depcmdpar2 {
        self.depcmdpar2(1)
    }
    #[doc = "0xc820 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn dep2cmdpar2(&self) -> &Depcmdpar2 {
        self.depcmdpar2(2)
    }
    #[doc = "0xc830 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn dep3cmdpar2(&self) -> &Depcmdpar2 {
        self.depcmdpar2(3)
    }
    #[doc = "0xc840 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn dep4cmdpar2(&self) -> &Depcmdpar2 {
        self.depcmdpar2(4)
    }
    #[doc = "0xc850 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn dep5cmdpar2(&self) -> &Depcmdpar2 {
        self.depcmdpar2(5)
    }
    #[doc = "0xc860 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn dep6cmdpar2(&self) -> &Depcmdpar2 {
        self.depcmdpar2(6)
    }
    #[doc = "0xc870 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn dep7cmdpar2(&self) -> &Depcmdpar2 {
        self.depcmdpar2(7)
    }
    #[doc = "0xc880 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn dep8cmdpar2(&self) -> &Depcmdpar2 {
        self.depcmdpar2(8)
    }
    #[doc = "0xc890 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn dep9cmdpar2(&self) -> &Depcmdpar2 {
        self.depcmdpar2(9)
    }
    #[doc = "0xc8a0 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn dep10cmdpar2(&self) -> &Depcmdpar2 {
        self.depcmdpar2(10)
    }
    #[doc = "0xc8b0 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn dep11cmdpar2(&self) -> &Depcmdpar2 {
        self.depcmdpar2(11)
    }
    #[doc = "0xc8c0 - Device Physical Endpoint-n Command Parameter 2 Register"]
    #[inline(always)]
    pub const fn dep12cmdpar2(&self) -> &Depcmdpar2 {
        self.depcmdpar2(12)
    }
    #[doc = "0xc804..0xc838 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn depcmdpar1(&self, n: usize) -> &Depcmdpar1 {
        #[allow(clippy::no_effect)]
        [(); 13][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(51204)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc804..0xc838 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub fn depcmdpar1_iter(&self) -> impl Iterator<Item = &Depcmdpar1> {
        (0..13).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(51204)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0xc804 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn dep0cmdpar1(&self) -> &Depcmdpar1 {
        self.depcmdpar1(0)
    }
    #[doc = "0xc814 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn dep1cmdpar1(&self) -> &Depcmdpar1 {
        self.depcmdpar1(1)
    }
    #[doc = "0xc824 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn dep2cmdpar1(&self) -> &Depcmdpar1 {
        self.depcmdpar1(2)
    }
    #[doc = "0xc834 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn dep3cmdpar1(&self) -> &Depcmdpar1 {
        self.depcmdpar1(3)
    }
    #[doc = "0xc844 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn dep4cmdpar1(&self) -> &Depcmdpar1 {
        self.depcmdpar1(4)
    }
    #[doc = "0xc854 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn dep5cmdpar1(&self) -> &Depcmdpar1 {
        self.depcmdpar1(5)
    }
    #[doc = "0xc864 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn dep6cmdpar1(&self) -> &Depcmdpar1 {
        self.depcmdpar1(6)
    }
    #[doc = "0xc874 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn dep7cmdpar1(&self) -> &Depcmdpar1 {
        self.depcmdpar1(7)
    }
    #[doc = "0xc884 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn dep8cmdpar1(&self) -> &Depcmdpar1 {
        self.depcmdpar1(8)
    }
    #[doc = "0xc894 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn dep9cmdpar1(&self) -> &Depcmdpar1 {
        self.depcmdpar1(9)
    }
    #[doc = "0xc8a4 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn dep10cmdpar1(&self) -> &Depcmdpar1 {
        self.depcmdpar1(10)
    }
    #[doc = "0xc8b4 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn dep11cmdpar1(&self) -> &Depcmdpar1 {
        self.depcmdpar1(11)
    }
    #[doc = "0xc8c4 - Device Physical Endpoint-n Command Parameter 1 Register"]
    #[inline(always)]
    pub const fn dep12cmdpar1(&self) -> &Depcmdpar1 {
        self.depcmdpar1(12)
    }
    #[doc = "0xc808..0xc83c - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn depcmdpar0(&self, n: usize) -> &Depcmdpar0 {
        #[allow(clippy::no_effect)]
        [(); 13][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(51208)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc808..0xc83c - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub fn depcmdpar0_iter(&self) -> impl Iterator<Item = &Depcmdpar0> {
        (0..13).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(51208)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0xc808 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn dep0cmdpar0(&self) -> &Depcmdpar0 {
        self.depcmdpar0(0)
    }
    #[doc = "0xc818 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn dep1cmdpar0(&self) -> &Depcmdpar0 {
        self.depcmdpar0(1)
    }
    #[doc = "0xc828 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn dep2cmdpar0(&self) -> &Depcmdpar0 {
        self.depcmdpar0(2)
    }
    #[doc = "0xc838 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn dep3cmdpar0(&self) -> &Depcmdpar0 {
        self.depcmdpar0(3)
    }
    #[doc = "0xc848 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn dep4cmdpar0(&self) -> &Depcmdpar0 {
        self.depcmdpar0(4)
    }
    #[doc = "0xc858 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn dep5cmdpar0(&self) -> &Depcmdpar0 {
        self.depcmdpar0(5)
    }
    #[doc = "0xc868 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn dep6cmdpar0(&self) -> &Depcmdpar0 {
        self.depcmdpar0(6)
    }
    #[doc = "0xc878 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn dep7cmdpar0(&self) -> &Depcmdpar0 {
        self.depcmdpar0(7)
    }
    #[doc = "0xc888 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn dep8cmdpar0(&self) -> &Depcmdpar0 {
        self.depcmdpar0(8)
    }
    #[doc = "0xc898 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn dep9cmdpar0(&self) -> &Depcmdpar0 {
        self.depcmdpar0(9)
    }
    #[doc = "0xc8a8 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn dep10cmdpar0(&self) -> &Depcmdpar0 {
        self.depcmdpar0(10)
    }
    #[doc = "0xc8b8 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn dep11cmdpar0(&self) -> &Depcmdpar0 {
        self.depcmdpar0(11)
    }
    #[doc = "0xc8c8 - Device Physical Endpoint-n Command Parameter 0 Register"]
    #[inline(always)]
    pub const fn dep12cmdpar0(&self) -> &Depcmdpar0 {
        self.depcmdpar0(12)
    }
    #[doc = "0xc80c..0xc840 - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn depcmd(&self, n: usize) -> &Depcmd {
        #[allow(clippy::no_effect)]
        [(); 13][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(51212)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc80c..0xc840 - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub fn depcmd_iter(&self) -> impl Iterator<Item = &Depcmd> {
        (0..13).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(51212)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0xc80c - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn dep0cmd(&self) -> &Depcmd {
        self.depcmd(0)
    }
    #[doc = "0xc81c - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn dep1cmd(&self) -> &Depcmd {
        self.depcmd(1)
    }
    #[doc = "0xc82c - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn dep2cmd(&self) -> &Depcmd {
        self.depcmd(2)
    }
    #[doc = "0xc83c - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn dep3cmd(&self) -> &Depcmd {
        self.depcmd(3)
    }
    #[doc = "0xc84c - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn dep4cmd(&self) -> &Depcmd {
        self.depcmd(4)
    }
    #[doc = "0xc85c - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn dep5cmd(&self) -> &Depcmd {
        self.depcmd(5)
    }
    #[doc = "0xc86c - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn dep6cmd(&self) -> &Depcmd {
        self.depcmd(6)
    }
    #[doc = "0xc87c - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn dep7cmd(&self) -> &Depcmd {
        self.depcmd(7)
    }
    #[doc = "0xc88c - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn dep8cmd(&self) -> &Depcmd {
        self.depcmd(8)
    }
    #[doc = "0xc89c - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn dep9cmd(&self) -> &Depcmd {
        self.depcmd(9)
    }
    #[doc = "0xc8ac - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn dep10cmd(&self) -> &Depcmd {
        self.depcmd(10)
    }
    #[doc = "0xc8bc - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn dep11cmd(&self) -> &Depcmd {
        self.depcmd(11)
    }
    #[doc = "0xc8cc - Device Physical Endpoint-n Command Register"]
    #[inline(always)]
    pub const fn dep12cmd(&self) -> &Depcmd {
        self.depcmd(12)
    }
}
#[doc = "GSBUSCFG0 (rw) register accessor: Global SoC Bus Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gsbuscfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gsbuscfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsbuscfg0`]
module"]
#[doc(alias = "GSBUSCFG0")]
pub type Gsbuscfg0 = crate::Reg<gsbuscfg0::Gsbuscfg0Spec>;
#[doc = "Global SoC Bus Configuration Register 0"]
pub mod gsbuscfg0;
#[doc = "GSBUSCFG1 (rw) register accessor: Global SoC Bus Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gsbuscfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gsbuscfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsbuscfg1`]
module"]
#[doc(alias = "GSBUSCFG1")]
pub type Gsbuscfg1 = crate::Reg<gsbuscfg1::Gsbuscfg1Spec>;
#[doc = "Global SoC Bus Configuration Register 1"]
pub mod gsbuscfg1;
#[doc = "GTXTHRCFG (rw) register accessor: Global Tx Threshold Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtxthrcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtxthrcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtxthrcfg`]
module"]
#[doc(alias = "GTXTHRCFG")]
pub type Gtxthrcfg = crate::Reg<gtxthrcfg::GtxthrcfgSpec>;
#[doc = "Global Tx Threshold Control Register"]
pub mod gtxthrcfg;
#[doc = "GRXTHRCFG (rw) register accessor: Global Rx Threshold Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxthrcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grxthrcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxthrcfg`]
module"]
#[doc(alias = "GRXTHRCFG")]
pub type Grxthrcfg = crate::Reg<grxthrcfg::GrxthrcfgSpec>;
#[doc = "Global Rx Threshold Control Register"]
pub mod grxthrcfg;
#[doc = "GCTL (rw) register accessor: Global Core Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gctl`]
module"]
#[doc(alias = "GCTL")]
pub type Gctl = crate::Reg<gctl::GctlSpec>;
#[doc = "Global Core Control Register"]
pub mod gctl;
#[doc = "GPMSTS (rw) register accessor: Global Power Management Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpmsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpmsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpmsts`]
module"]
#[doc(alias = "GPMSTS")]
pub type Gpmsts = crate::Reg<gpmsts::GpmstsSpec>;
#[doc = "Global Power Management Status Register"]
pub mod gpmsts;
#[doc = "GSTS (rw) register accessor: Global Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsts`]
module"]
#[doc(alias = "GSTS")]
pub type Gsts = crate::Reg<gsts::GstsSpec>;
#[doc = "Global Status Register"]
pub mod gsts;
#[doc = "GUCTL1 (rw) register accessor: Global User Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`guctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`guctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@guctl1`]
module"]
#[doc(alias = "GUCTL1")]
pub type Guctl1 = crate::Reg<guctl1::Guctl1Spec>;
#[doc = "Global User Control Register 1"]
pub mod guctl1;
#[doc = "GSNPSID (r) register accessor: Global SNPS ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gsnpsid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsnpsid`]
module"]
#[doc(alias = "GSNPSID")]
pub type Gsnpsid = crate::Reg<gsnpsid::GsnpsidSpec>;
#[doc = "Global SNPS ID Register"]
pub mod gsnpsid;
#[doc = "GGPIO (rw) register accessor: Global General Purpose Input/Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ggpio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ggpio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ggpio`]
module"]
#[doc(alias = "GGPIO")]
pub type Ggpio = crate::Reg<ggpio::GgpioSpec>;
#[doc = "Global General Purpose Input/Output Register"]
pub mod ggpio;
#[doc = "GUID (rw) register accessor: Global User ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`guid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`guid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@guid`]
module"]
#[doc(alias = "GUID")]
pub type Guid = crate::Reg<guid::GuidSpec>;
#[doc = "Global User ID Register"]
pub mod guid;
#[doc = "GUCTL (rw) register accessor: Global User Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`guctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`guctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@guctl`]
module"]
#[doc(alias = "GUCTL")]
pub type Guctl = crate::Reg<guctl::GuctlSpec>;
#[doc = "Global User Control Register"]
pub mod guctl;
#[doc = "GBUSERRADDRLO (r) register accessor: Global SoC Bus Error Address Register - Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gbuserraddrlo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gbuserraddrlo`]
module"]
#[doc(alias = "GBUSERRADDRLO")]
pub type Gbuserraddrlo = crate::Reg<gbuserraddrlo::GbuserraddrloSpec>;
#[doc = "Global SoC Bus Error Address Register - Low"]
pub mod gbuserraddrlo;
#[doc = "GBUSERRADDRHI (r) register accessor: Global SoC Bus Error Address Register - High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gbuserraddrhi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gbuserraddrhi`]
module"]
#[doc(alias = "GBUSERRADDRHI")]
pub type Gbuserraddrhi = crate::Reg<gbuserraddrhi::GbuserraddrhiSpec>;
#[doc = "Global SoC Bus Error Address Register - High"]
pub mod gbuserraddrhi;
#[doc = "GPRTBIMAPLO (rw) register accessor: Global SS Port to Bus Instance Mapping Register - Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gprtbimaplo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gprtbimaplo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gprtbimaplo`]
module"]
#[doc(alias = "GPRTBIMAPLO")]
pub type Gprtbimaplo = crate::Reg<gprtbimaplo::GprtbimaploSpec>;
#[doc = "Global SS Port to Bus Instance Mapping Register - Low"]
pub mod gprtbimaplo;
#[doc = "GHWPARAMS0 (r) register accessor: Global Hardware Parameters Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwparams0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghwparams0`]
module"]
#[doc(alias = "GHWPARAMS0")]
pub type Ghwparams0 = crate::Reg<ghwparams0::Ghwparams0Spec>;
#[doc = "Global Hardware Parameters Register 0"]
pub mod ghwparams0;
#[doc = "GHWPARAMS1 (r) register accessor: Global Hardware Parameters Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwparams1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghwparams1`]
module"]
#[doc(alias = "GHWPARAMS1")]
pub type Ghwparams1 = crate::Reg<ghwparams1::Ghwparams1Spec>;
#[doc = "Global Hardware Parameters Register 1"]
pub mod ghwparams1;
#[doc = "GHWPARAMS2 (r) register accessor: Global Hardware Parameters Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwparams2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghwparams2`]
module"]
#[doc(alias = "GHWPARAMS2")]
pub type Ghwparams2 = crate::Reg<ghwparams2::Ghwparams2Spec>;
#[doc = "Global Hardware Parameters Register 2"]
pub mod ghwparams2;
#[doc = "GHWPARAMS3 (r) register accessor: Global Hardware Parameters Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwparams3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghwparams3`]
module"]
#[doc(alias = "GHWPARAMS3")]
pub type Ghwparams3 = crate::Reg<ghwparams3::Ghwparams3Spec>;
#[doc = "Global Hardware Parameters Register 3"]
pub mod ghwparams3;
#[doc = "GHWPARAMS4 (r) register accessor: Global Hardware Parameters Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwparams4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghwparams4`]
module"]
#[doc(alias = "GHWPARAMS4")]
pub type Ghwparams4 = crate::Reg<ghwparams4::Ghwparams4Spec>;
#[doc = "Global Hardware Parameters Register 4"]
pub mod ghwparams4;
#[doc = "GHWPARAMS5 (r) register accessor: Global Hardware Parameters Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwparams5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghwparams5`]
module"]
#[doc(alias = "GHWPARAMS5")]
pub type Ghwparams5 = crate::Reg<ghwparams5::Ghwparams5Spec>;
#[doc = "Global Hardware Parameters Register 5"]
pub mod ghwparams5;
#[doc = "GHWPARAMS6 (r) register accessor: Global Hardware Parameters Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwparams6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghwparams6`]
module"]
#[doc(alias = "GHWPARAMS6")]
pub type Ghwparams6 = crate::Reg<ghwparams6::Ghwparams6Spec>;
#[doc = "Global Hardware Parameters Register 6"]
pub mod ghwparams6;
#[doc = "GHWPARAMS7 (r) register accessor: Global Hardware Parameters Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwparams7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghwparams7`]
module"]
#[doc(alias = "GHWPARAMS7")]
pub type Ghwparams7 = crate::Reg<ghwparams7::Ghwparams7Spec>;
#[doc = "Global Hardware Parameters Register 7"]
pub mod ghwparams7;
#[doc = "GDBGFIFOSPACE (rw) register accessor: Global Debug Queue/FIFO Space Available Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdbgfifospace::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdbgfifospace::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdbgfifospace`]
module"]
#[doc(alias = "GDBGFIFOSPACE")]
pub type Gdbgfifospace = crate::Reg<gdbgfifospace::GdbgfifospaceSpec>;
#[doc = "Global Debug Queue/FIFO Space Available Register"]
pub mod gdbgfifospace;
#[doc = "GDBGLTSSM (rw) register accessor: Global Debug LTSSM Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdbgltssm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdbgltssm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdbgltssm`]
module"]
#[doc(alias = "GDBGLTSSM")]
pub type Gdbgltssm = crate::Reg<gdbgltssm::GdbgltssmSpec>;
#[doc = "Global Debug LTSSM Register"]
pub mod gdbgltssm;
#[doc = "GDBGLNMCC (r) register accessor: Global Debug LNMCC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdbglnmcc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdbglnmcc`]
module"]
#[doc(alias = "GDBGLNMCC")]
pub type Gdbglnmcc = crate::Reg<gdbglnmcc::GdbglnmccSpec>;
#[doc = "Global Debug LNMCC Register"]
pub mod gdbglnmcc;
#[doc = "GDBGBMU (rw) register accessor: Global Debug BMU Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdbgbmu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdbgbmu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdbgbmu`]
module"]
#[doc(alias = "GDBGBMU")]
pub type Gdbgbmu = crate::Reg<gdbgbmu::GdbgbmuSpec>;
#[doc = "Global Debug BMU Register"]
pub mod gdbgbmu;
#[doc = "GDBGLSPMUX (rw) register accessor: Global Debug LSP MUX Register - Device\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdbglspmux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdbglspmux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdbglspmux`]
module"]
#[doc(alias = "GDBGLSPMUX")]
pub type Gdbglspmux = crate::Reg<gdbglspmux::GdbglspmuxSpec>;
#[doc = "Global Debug LSP MUX Register - Device"]
pub mod gdbglspmux;
#[doc = "GDBGLSP (r) register accessor: Global Debug LSP Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdbglsp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdbglsp`]
module"]
#[doc(alias = "GDBGLSP")]
pub type Gdbglsp = crate::Reg<gdbglsp::GdbglspSpec>;
#[doc = "Global Debug LSP Register"]
pub mod gdbglsp;
#[doc = "GDBGEPINFO0 (r) register accessor: Global Debug Endpoint Information Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdbgepinfo0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdbgepinfo0`]
module"]
#[doc(alias = "GDBGEPINFO0")]
pub type Gdbgepinfo0 = crate::Reg<gdbgepinfo0::Gdbgepinfo0Spec>;
#[doc = "Global Debug Endpoint Information Register 0"]
pub mod gdbgepinfo0;
#[doc = "GDBGEPINFO1 (r) register accessor: Global Debug Endpoint Information Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdbgepinfo1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdbgepinfo1`]
module"]
#[doc(alias = "GDBGEPINFO1")]
pub type Gdbgepinfo1 = crate::Reg<gdbgepinfo1::Gdbgepinfo1Spec>;
#[doc = "Global Debug Endpoint Information Register 1"]
pub mod gdbgepinfo1;
#[doc = "GPRTBIMAP_HSLO (rw) register accessor: Global High-Speed Port to Bus Instance Mapping Register - Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gprtbimap_hslo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gprtbimap_hslo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gprtbimap_hslo`]
module"]
#[doc(alias = "GPRTBIMAP_HSLO")]
pub type GprtbimapHslo = crate::Reg<gprtbimap_hslo::GprtbimapHsloSpec>;
#[doc = "Global High-Speed Port to Bus Instance Mapping Register - Low"]
pub mod gprtbimap_hslo;
#[doc = "GPRTBIMAP_FSLO (rw) register accessor: Global Full-Speed Port to Bus Instance Mapping Register - Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gprtbimap_fslo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gprtbimap_fslo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gprtbimap_fslo`]
module"]
#[doc(alias = "GPRTBIMAP_FSLO")]
pub type GprtbimapFslo = crate::Reg<gprtbimap_fslo::GprtbimapFsloSpec>;
#[doc = "Global Full-Speed Port to Bus Instance Mapping Register - Low"]
pub mod gprtbimap_fslo;
#[doc = "GUSB2PHYCFG0 (rw) register accessor: Global USB2 PHY Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusb2phycfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusb2phycfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gusb2phycfg0`]
module"]
#[doc(alias = "GUSB2PHYCFG0")]
pub type Gusb2phycfg0 = crate::Reg<gusb2phycfg0::Gusb2phycfg0Spec>;
#[doc = "Global USB2 PHY Configuration Register 0"]
pub mod gusb2phycfg0;
#[doc = "GUSB3PIPECTL0 (rw) register accessor: Global USB3 PIPE Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusb3pipectl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusb3pipectl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gusb3pipectl0`]
module"]
#[doc(alias = "GUSB3PIPECTL0")]
pub type Gusb3pipectl0 = crate::Reg<gusb3pipectl0::Gusb3pipectl0Spec>;
#[doc = "Global USB3 PIPE Control Register 0"]
pub mod gusb3pipectl0;
#[doc = "GTXFIFOSIZ (rw) register accessor: Global Transmit FIFO Size Register n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtxfifosiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtxfifosiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtxfifosiz`]
module"]
#[doc(alias = "GTXFIFOSIZ")]
pub type Gtxfifosiz = crate::Reg<gtxfifosiz::GtxfifosizSpec>;
#[doc = "Global Transmit FIFO Size Register n"]
pub mod gtxfifosiz;
#[doc = "GRXFIFOSIZ (rw) register accessor: Global Receive FIFO Size Register n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxfifosiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grxfifosiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxfifosiz`]
module"]
#[doc(alias = "GRXFIFOSIZ")]
pub type Grxfifosiz = crate::Reg<grxfifosiz::GrxfifosizSpec>;
#[doc = "Global Receive FIFO Size Register n"]
pub mod grxfifosiz;
#[doc = "GEVNTADRLO0 (rw) register accessor: Global Event Buffer Address (Low) Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gevntadrlo0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gevntadrlo0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gevntadrlo0`]
module"]
#[doc(alias = "GEVNTADRLO0")]
pub type Gevntadrlo0 = crate::Reg<gevntadrlo0::Gevntadrlo0Spec>;
#[doc = "Global Event Buffer Address (Low) Register 0"]
pub mod gevntadrlo0;
#[doc = "GEVNTADRHI0 (rw) register accessor: Global Event Buffer Address (High) Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gevntadrhi0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gevntadrhi0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gevntadrhi0`]
module"]
#[doc(alias = "GEVNTADRHI0")]
pub type Gevntadrhi0 = crate::Reg<gevntadrhi0::Gevntadrhi0Spec>;
#[doc = "Global Event Buffer Address (High) Register 0"]
pub mod gevntadrhi0;
#[doc = "GEVNTSIZ0 (rw) register accessor: Global Event Buffer Size Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gevntsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gevntsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gevntsiz0`]
module"]
#[doc(alias = "GEVNTSIZ0")]
pub type Gevntsiz0 = crate::Reg<gevntsiz0::Gevntsiz0Spec>;
#[doc = "Global Event Buffer Size Register 0"]
pub mod gevntsiz0;
#[doc = "GEVNTCOUNT0 (rw) register accessor: Global Event Buffer Count Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gevntcount0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gevntcount0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gevntcount0`]
module"]
#[doc(alias = "GEVNTCOUNT0")]
pub type Gevntcount0 = crate::Reg<gevntcount0::Gevntcount0Spec>;
#[doc = "Global Event Buffer Count Register 0"]
pub mod gevntcount0;
#[doc = "GHWPARAMS8 (r) register accessor: Global Hardware Parameters Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwparams8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghwparams8`]
module"]
#[doc(alias = "GHWPARAMS8")]
pub type Ghwparams8 = crate::Reg<ghwparams8::Ghwparams8Spec>;
#[doc = "Global Hardware Parameters Register 8"]
pub mod ghwparams8;
#[doc = "GTXFIFOPRIDEV (rw) register accessor: Global Device TX FIFO DMA Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtxfifopridev::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtxfifopridev::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtxfifopridev`]
module"]
#[doc(alias = "GTXFIFOPRIDEV")]
pub type Gtxfifopridev = crate::Reg<gtxfifopridev::GtxfifopridevSpec>;
#[doc = "Global Device TX FIFO DMA Priority Register"]
pub mod gtxfifopridev;
#[doc = "GTXFIFOPRIHST (rw) register accessor: Global Host TX FIFO DMA Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtxfifoprihst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtxfifoprihst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtxfifoprihst`]
module"]
#[doc(alias = "GTXFIFOPRIHST")]
pub type Gtxfifoprihst = crate::Reg<gtxfifoprihst::GtxfifoprihstSpec>;
#[doc = "Global Host TX FIFO DMA Priority Register"]
pub mod gtxfifoprihst;
#[doc = "GRXFIFOPRIHST (rw) register accessor: Global Host RX FIFO DMA Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxfifoprihst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grxfifoprihst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxfifoprihst`]
module"]
#[doc(alias = "GRXFIFOPRIHST")]
pub type Grxfifoprihst = crate::Reg<grxfifoprihst::GrxfifoprihstSpec>;
#[doc = "Global Host RX FIFO DMA Priority Register"]
pub mod grxfifoprihst;
#[doc = "GFIFOPRIDBC (rw) register accessor: Global Host Debug Capability DMA Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gfifopridbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gfifopridbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gfifopridbc`]
module"]
#[doc(alias = "GFIFOPRIDBC")]
pub type Gfifopridbc = crate::Reg<gfifopridbc::GfifopridbcSpec>;
#[doc = "Global Host Debug Capability DMA Priority Register"]
pub mod gfifopridbc;
#[doc = "GDMAHLRATIO (rw) register accessor: Global Host FIFO DMA High-Low Priority Ratio Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdmahlratio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdmahlratio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdmahlratio`]
module"]
#[doc(alias = "GDMAHLRATIO")]
pub type Gdmahlratio = crate::Reg<gdmahlratio::GdmahlratioSpec>;
#[doc = "Global Host FIFO DMA High-Low Priority Ratio Register"]
pub mod gdmahlratio;
#[doc = "GFLADJ (rw) register accessor: Global Frame Length Adjustment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gfladj::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gfladj::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gfladj`]
module"]
#[doc(alias = "GFLADJ")]
pub type Gfladj = crate::Reg<gfladj::GfladjSpec>;
#[doc = "Global Frame Length Adjustment Register"]
pub mod gfladj;
#[doc = "DCFG (rw) register accessor: Device Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`]
module"]
#[doc(alias = "DCFG")]
pub type Dcfg = crate::Reg<dcfg::DcfgSpec>;
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: Device Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`]
module"]
#[doc(alias = "DCTL")]
pub type Dctl = crate::Reg<dctl::DctlSpec>;
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "DEVTEN (rw) register accessor: Device Event Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devten`]
module"]
#[doc(alias = "DEVTEN")]
pub type Devten = crate::Reg<devten::DevtenSpec>;
#[doc = "Device Event Enable Register"]
pub mod devten;
#[doc = "DSTS (r) register accessor: Device Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsts`]
module"]
#[doc(alias = "DSTS")]
pub type Dsts = crate::Reg<dsts::DstsSpec>;
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "DGCMDPAR (rw) register accessor: Device Generic Command Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dgcmdpar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dgcmdpar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dgcmdpar`]
module"]
#[doc(alias = "DGCMDPAR")]
pub type Dgcmdpar = crate::Reg<dgcmdpar::DgcmdparSpec>;
#[doc = "Device Generic Command Parameter Register"]
pub mod dgcmdpar;
#[doc = "DGCMD (rw) register accessor: Device Generic Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dgcmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dgcmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dgcmd`]
module"]
#[doc(alias = "DGCMD")]
pub type Dgcmd = crate::Reg<dgcmd::DgcmdSpec>;
#[doc = "Device Generic Command Register"]
pub mod dgcmd;
#[doc = "DALEPENA (rw) register accessor: Device Active USB Endpoint Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dalepena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dalepena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dalepena`]
module"]
#[doc(alias = "DALEPENA")]
pub type Dalepena = crate::Reg<dalepena::DalepenaSpec>;
#[doc = "Device Active USB Endpoint Enable Register"]
pub mod dalepena;
#[doc = "DEPCMDPAR2 (rw) register accessor: Device Physical Endpoint-n Command Parameter 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`depcmdpar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`depcmdpar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@depcmdpar2`]
module"]
#[doc(alias = "DEPCMDPAR2")]
pub type Depcmdpar2 = crate::Reg<depcmdpar2::Depcmdpar2Spec>;
#[doc = "Device Physical Endpoint-n Command Parameter 2 Register"]
pub mod depcmdpar2;
#[doc = "DEPCMDPAR1 (rw) register accessor: Device Physical Endpoint-n Command Parameter 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`depcmdpar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`depcmdpar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@depcmdpar1`]
module"]
#[doc(alias = "DEPCMDPAR1")]
pub type Depcmdpar1 = crate::Reg<depcmdpar1::Depcmdpar1Spec>;
#[doc = "Device Physical Endpoint-n Command Parameter 1 Register"]
pub mod depcmdpar1;
#[doc = "DEPCMDPAR0 (rw) register accessor: Device Physical Endpoint-n Command Parameter 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`depcmdpar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`depcmdpar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@depcmdpar0`]
module"]
#[doc(alias = "DEPCMDPAR0")]
pub type Depcmdpar0 = crate::Reg<depcmdpar0::Depcmdpar0Spec>;
#[doc = "Device Physical Endpoint-n Command Parameter 0 Register"]
pub mod depcmdpar0;
#[doc = "DEPCMD (rw) register accessor: Device Physical Endpoint-n Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`depcmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`depcmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@depcmd`]
module"]
#[doc(alias = "DEPCMD")]
pub type Depcmd = crate::Reg<depcmd::DepcmdSpec>;
#[doc = "Device Physical Endpoint-n Command Register"]
pub mod depcmd;
